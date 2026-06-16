use std::path::PathBuf;
use std::{env, fs};

use shader_slang::{CompileTarget, CompilerOptions, SessionDesc};
use shader_slang::{GlobalSession, TargetDesc};

mod reflect;
mod tokens;

fn main() {
    let dir = PathBuf::from_iter([
        env::args().skip(1).next().unwrap().as_str(),
        "src",
        "shaders",
    ]);

    let bin_dir = dir.join("bin");

    let mut old_files = fs::read_dir(&bin_dir)
        .unwrap()
        .map(|file| file.unwrap().path())
        .filter(|file| file.extension().map_or(false, |ext| ext == "spv"))
        .collect::<Vec<_>>();

    let global_session = GlobalSession::new().unwrap();

    let target_desc = [TargetDesc::default()
        .format(CompileTarget::Spirv)
        .profile(global_session.find_profile("spirv_1_5"))];

    let compiler_options =
        CompilerOptions::default().optimization(shader_slang::OptimizationLevel::High);

    let session_desc = SessionDesc::default()
        .targets(&target_desc)
        .options(&compiler_options);

    let session = global_session.create_session(&session_desc).unwrap();

    let mut mod_code = quote::quote! {
        use ash::vk::Format;
        use ash::vk::VertexInputAttributeDescription;
        use ash::vk::VertexInputBindingDescription;
        use ash::vk::VertexInputRate;

        pub struct SlangShader {
            pub code_bytes: &'static [u8],

            pub attributes: &'static [VertexInputAttributeDescription],

            pub bindings: &'static [VertexInputBindingDescription],
        }

        impl SlangShader {
            pub fn code(&self) -> &'static [u32] {
                unsafe {
                    std::slice::from_raw_parts(self.code_bytes.as_ptr() as _, self.code_bytes.len() / 4)
                }
            }
        }
    };

    _ = fs::create_dir(&bin_dir);

    for file in fs::read_dir(dir.join("src"))
        .unwrap()
        .map(|file| file.unwrap().path())
        .filter(|file| file.extension().map_or(false, |ext| ext == "slang"))
    {
        let module = session.load_module(file.to_str().unwrap()).unwrap();

        let entry_point = module.find_entry_point_by_name("main").unwrap();

        let component = session
            .create_composite_component_type(&[module.into(), entry_point.into()])
            .unwrap();

        let linked_component = component.link().unwrap();

        let blob = linked_component.entry_point_code(0, 0).unwrap();

        let code = blob.as_slice();

        let sb = fs::read_to_string(file.with_extension("sb")).ok();

        let shader_mod_code =
            reflect::reflect(code, sb, file.file_stem().unwrap().to_str().unwrap());

        mod_code.extend(shader_mod_code);

        let bin_file = bin_dir
            .join(file.file_name().unwrap())
            .with_extension("spv");

        fs::write(&bin_file, code).unwrap();

        old_files.retain(|old_file| old_file != &bin_file);
    }

    old_files
        .iter()
        .for_each(|file| fs::remove_file(file).unwrap());

    let syntax = syn::parse_file(&mod_code.to_string()).unwrap();

    fs::write(dir.join("mod.rs"), prettyplease::unparse(&syntax)).unwrap();
}
