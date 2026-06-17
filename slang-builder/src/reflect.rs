use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use spirv_cross2::{
    Compiler, Module,
    reflect::{BitWidth, Resource, ResourceType, ScalarKind, ShaderResources, TypeInner},
    spirv::Decoration,
    targets,
};

use crate::tokens::{DescriptorBinding, VertexAttribute, VertexBinding};

pub fn reflect(code: &[u8], sb: Option<String>, name: &str) -> TokenStream {
    let (_, words, _) = unsafe { code.align_to::<u32>() };

    unsafe { std::mem::transmute::<&[u8], &[u32]>(code) };

    let cross_module = Module::from_words(words);

    let compiler = Compiler::<targets::None>::new(cross_module).unwrap();

    let resources = compiler.shader_resources().unwrap();

    let vertex_input = vertex_input(&compiler, &resources, sb);

    let descriptor_sets = get_res1(&compiler, &resources);

    let name_upper = TokenStream::from_str(&name.to_ascii_uppercase()).unwrap();

    let spv_path = format!("bin/{}.spv", name);

    let bindings = vertex_input.0;

    let attributes = vertex_input.1;

    let descriptor_bindings = descriptor_sets
        .iter()
        .map(|set| {
            quote! {
                &[#(#set),*]
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub const #name_upper: CompiledShader = CompiledShader {
            code: include_bytes!(#spv_path),

            vertex_bindings: &[#(#bindings),*],

            vertex_attributes: &[#(#attributes),*],

            set_layouts: &[#(#descriptor_bindings),*],
        };
    }
}

fn vertex_input(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
    sb: Option<String>,
) -> (Vec<VertexBinding>, Vec<VertexAttribute>) {
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

    let sb_iter = match sb.as_ref() {
        None => vec![(inputs.len() as u32, "VERTEX")],
        Some(str) => str
            .trim_end_matches(|c: char| c.is_whitespace())
            .split('\n')
            .map(|line| {
                let (range_str, rate_str) = line.split_once(' ').unwrap();

                let range = range_str.parse::<u32>().unwrap();

                let rate = match rate_str {
                    "vertex" => "VERTEX",
                    "instance" => "INSTANCE",
                    _ => panic!("expected vertex or instance"),
                };

                (range, rate)
            })
            .collect::<Vec<_>>(),
    };

    let mut inputs_iter = inputs.iter();

    sb_iter.iter().enumerate().fold(
        (vec![], vec![]),
        |(mut bindings, mut attributes), (binding, (range, rate))| {
            let offset =
                inputs_iter
                    .by_ref()
                    .take(*range as usize)
                    .fold(0, |offset, (location, input)| {
                        let (format, size) =
                            to_format(compiler.type_description(input.base_type_id).unwrap().inner);

                        attributes.push(VertexAttribute {
                            location: *location as u32,
                            binding: binding as u32,
                            format,
                            offset,
                        });

                        offset + size
                    });

            bindings.push(VertexBinding {
                binding: binding as u32,
                stride: offset,
                input_rate: rate,
            });

            (bindings, attributes)
        },
    )
}

fn get_res1(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
) -> Vec<Vec<DescriptorBinding>> {
    let mut ress = vec![];

    ress.extend(get_resources(
        compiler,
        resources,
        ResourceType::SampledImage,
    ));

    ress.sort_by_key(|resource| resource.0 << 16 | resource.1);

    let mut result = vec![];

    let mut ress_iter = ress.iter();

    let mut set = 0;

    loop {
        let bind = ress_iter
            .by_ref()
            .take_while(|resource| resource.0 == set)
            .map(|resource| DescriptorBinding {
                binding: resource.1,
                descriptor_type: "COMBINED_IMAGE_SAMPLER",
                descriptor_count: 1,
                stage_flags: "FRAGMENT",
            })
            .collect::<Vec<_>>();

        if bind.is_empty() {
            break;
        }

        result.push(bind);

        set += 1;
    }

    result
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
