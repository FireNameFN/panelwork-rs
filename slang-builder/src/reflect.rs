use std::sync::LazyLock;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;
use spirv_cross2::{
    Compiler, Module,
    reflect::{BitWidth, ResourceType, ScalarKind, ShaderResources, TypeInner},
    spirv::{Decoration, ExecutionModel},
    targets,
};

use crate::tokens::{DescriptorBinding, VertexAttribute, VertexBinding};

const REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("// sb/(vertex|instance)").unwrap());

pub fn reflect(code: &[u8], content: String, name: &str) -> TokenStream {
    let (_, words, _) = unsafe { code.align_to::<u32>() };

    unsafe { std::mem::transmute::<&[u8], &[u32]>(code) };

    let cross_module = Module::from_words(words);

    let compiler = Compiler::<targets::None>::new(cross_module).unwrap();

    let model = compiler.execution_model().unwrap();

    let resources = compiler.shader_resources().unwrap();

    let vertex_input = match model {
        ExecutionModel::Vertex => vertex_input(&compiler, &resources, content),
        _ => (vec![], vec![]),
    };

    let descriptor_sets = get_descriptors(&compiler, &resources);

    let name_upper = Ident::new(&name.to_ascii_uppercase(), Span::call_site());

    let spv_path = format!("bin/{}.spv", name);

    let bindings = vertex_input.0;

    let attributes = vertex_input.1;

    quote! {
        pub const #name_upper: CompiledShader = CompiledShader {
            code: include_bytes!(#spv_path),

            vertex_bindings: &[#(#bindings),*],

            vertex_attributes: &[#(#attributes),*],

            set_layouts: &[#(&[#(#descriptor_sets),*]),*],
        };
    }
}

fn vertex_input(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
    content: String,
) -> (Vec<VertexBinding>, Vec<VertexAttribute>) {
    #[derive(Clone, PartialEq)]
    enum Rate {
        Invalid,
        Vertex,
        Instance,
    }

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

    let captures = REGEX.captures_iter(&content).collect::<Vec<_>>();

    let sb_iter = if captures.len() == inputs.len() {
        captures
            .iter()
            .map(|capture| match capture.get(1).unwrap().as_str() {
                "vertex" => Rate::Vertex,
                "instance" => Rate::Instance,
                _ => unreachable!(),
            })
            .collect()
    } else if captures.len() == 0 {
        vec![Rate::Vertex; inputs.len()]
    } else {
        panic!("wrong count of sb")
    };

    let mut bindings = Vec::with_capacity(inputs.len());

    let mut attributes = Vec::with_capacity(inputs.len());

    let mut cur_rate = Rate::Invalid;

    let mut binding = 0;

    for ((location, input), rate) in inputs.iter().zip(sb_iter) {
        if cur_rate != rate {
            binding = bindings.len() as u32;

            bindings.push(VertexBinding {
                binding: binding,
                stride: 0,
                input_rate: match rate {
                    Rate::Vertex => "VERTEX",
                    Rate::Instance => "INSTANCE",
                    _ => unreachable!(),
                },
            });

            cur_rate = rate;
        }

        let (format, size) =
            to_format(compiler.type_description(input.base_type_id).unwrap().inner);

        let stride = &mut bindings.last_mut().unwrap().stride;

        attributes.push(VertexAttribute {
            location: *location,
            binding,
            format,
            offset: *stride,
        });

        *stride += size;
    }

    (bindings, attributes)
}

fn get_descriptors(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
) -> Vec<Vec<DescriptorBinding>> {
    let mut ress = vec![];

    ress.extend(get_resources(
        compiler,
        resources,
        ResourceType::SampledImage,
        "COMBINED_IMAGE_SAMPLER",
    ));

    ress.sort_by_key(|descriptor| descriptor.set << 16 | descriptor.binding);

    let mut result = vec![];

    let mut ress_iter = ress.iter();

    let mut set = 0;

    loop {
        let bind = ress_iter
            .by_ref()
            .take_while(|descriptor| descriptor.set == set)
            .map(|descriptor| DescriptorBinding {
                binding: descriptor.binding,
                descriptor_type: descriptor.descriptor_type,
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

struct DescriptorResource {
    set: u32,
    binding: u32,
    descriptor_type: &'static str,
}

fn get_resources<'a>(
    compiler: &Compiler<targets::None>,
    resources: &ShaderResources,
    resource_type: ResourceType,
    descriptor_type: &'static str,
) -> impl Iterator<Item = DescriptorResource> {
    resources
        .resources_for_type(resource_type)
        .unwrap()
        .map(move |resource| DescriptorResource {
            set: compiler
                .decoration(resource.id, Decoration::DescriptorSet)
                .unwrap()
                .unwrap()
                .as_literal()
                .unwrap(),
            binding: compiler
                .decoration(resource.id, Decoration::Binding)
                .unwrap()
                .unwrap()
                .as_literal()
                .unwrap(),
            descriptor_type,
        })
}

fn to_format(type_inner: TypeInner) -> (String, u32) {
    let (width, scalar) = match type_inner {
        TypeInner::Scalar(scalar) => (1, scalar),
        TypeInner::Vector { width, scalar } => (width, scalar),
        _ => panic!("unsupported type"),
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
        _ => panic!("unsupported kind"),
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
