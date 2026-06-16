use std::str::FromStr;

use proc_macro2::TokenStream;
use spirv_cross2::{
    Compiler, Module,
    reflect::{BitWidth, Resource, ResourceType, ScalarKind, ShaderResources, TypeInner},
    spirv::Decoration,
    targets,
};

use crate::tokens::{VertexAttribute, VertexBinding};

pub fn reflect(code: &[u8], sb: Option<String>, name: &str) -> TokenStream {
    let (_, words, _) = unsafe { code.align_to::<u32>() };

    unsafe { std::mem::transmute::<&[u8], &[u32]>(code) };

    let cross_module = Module::from_words(words);

    let compiler = Compiler::<targets::None>::new(cross_module).unwrap();

    let resources = compiler.shader_resources().unwrap();

    let vertex_input = vertex_input(compiler, resources, sb);

    let name_upper = TokenStream::from_str(&name.to_ascii_uppercase()).unwrap();

    let spv_path = format!("bin/{}.spv", name);

    let attributes = vertex_input.0;

    let bindings = vertex_input.1;

    quote::quote! {
        pub const #name_upper: SlangShader = SlangShader {
            code_bytes: include_bytes!(#spv_path),

            attributes: &[#(#attributes),*],

            bindings: &[#(#bindings),*],
        };
    }
}

fn vertex_input(
    compiler: Compiler<targets::None>,
    resources: ShaderResources,
    sb: Option<String>,
) -> (Vec<VertexAttribute>, Vec<VertexBinding>) {
    let mut sb_iter = sb.as_ref().map(|str| {
        str.trim_end_matches(|c: char| c.is_whitespace())
            .split('\n')
            .map(|line| {
                let (range_str, rate_str) = line.split_once(' ').unwrap();

                let range = range_str.parse::<i32>().unwrap();

                let rate = match rate_str {
                    "vertex" => "VERTEX",
                    "instance" => "INSTANCE",
                    _ => panic!("expected vertex or instance"),
                };

                (range, rate)
            })
    });

    let mut offset = 0;

    let mut binding = 0;

    let mut binding_offset = 0;

    let mut sb_entry = sb_iter.as_mut().map(|sb| sb.next().unwrap());

    let mut inputs = resources
        .resources_for_type(ResourceType::StageInput)
        .unwrap()
        .map(|input| {
            (
                compiler
                    .decoration(input.id, Decoration::Location)
                    .unwrap()
                    .unwrap()
                    .as_literal()
                    .unwrap(),
                input,
            )
        })
        .collect::<Vec<_>>();

    inputs.sort_by_key(|input| input.0);

    let mut bindings = vec![];

    let attributes = inputs
        .iter()
        .map(|input| {
            let (format, size) = to_format(
                compiler
                    .type_description(input.1.base_type_id)
                    .unwrap()
                    .inner,
            );

            let attribute = VertexAttribute {
                location: input.0,
                binding,
                format,
                offset,
            };

            offset += size;

            binding_offset += 1;

            if sb_entry.is_some() && binding_offset >= sb_entry.unwrap().0 {
                bindings.push(VertexBinding {
                    binding: binding,
                    stride: offset,
                    input_rate: sb_entry.unwrap().1,
                });

                sb_entry = sb_iter.as_mut().unwrap().next();

                binding += 1;

                binding_offset = 0;
                offset = 0;
            }

            attribute
        })
        .collect::<Vec<_>>();

    (attributes, bindings)
}

fn get_resources<'a>(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
    resource_type: ResourceType,
) -> impl Iterator<Item = (u32, u32, Resource<'a>)> {
    resources
        .resources_for_type(resource_type)
        .unwrap()
        .map(|resource| {
            (
                compiler
                    .decoration(resource.id, Decoration::DescriptorSet)
                    .unwrap()
                    .unwrap()
                    .as_literal()
                    .unwrap(),
                compiler
                    .decoration(resource.id, Decoration::Binding)
                    .unwrap()
                    .unwrap()
                    .as_literal()
                    .unwrap(),
                resource,
            )
        })
}

fn to_format(type_inner: TypeInner) -> (String, u32) {
    let (width, scalar) = match type_inner {
        TypeInner::Scalar(scalar) => (1, scalar),
        TypeInner::Vector { width, scalar } => (width, scalar),
        _ => todo!(),
    };

    let size = match scalar.size {
        BitWidth::Bit => 1,
        BitWidth::Byte => 8,
        BitWidth::HalfWord => 16,
        BitWidth::Word => 32,
        BitWidth::DoubleWord => 64,
    };

    let kind_str = match scalar.kind {
        ScalarKind::Float => "SFLOAT",
        _ => todo!(),
    };

    (
        match width {
            1 => format!("R{size}_{kind_str}"),
            2 => format!("R{size}G{size}_{kind_str}"),
            3 => format!("R{size}G{size}B{size}_{kind_str}"),
            4 => format!("R{size}G{size}B{size}A{size}_{kind_str}"),
            _ => todo!(),
        },
        (width * size).div_ceil(8),
    )
}
