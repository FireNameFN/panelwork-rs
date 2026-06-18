use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};

use quote::quote;
use shader_slang::{CompileTarget, CompilerOptions, SessionDesc};
use shader_slang::{GlobalSession, TargetDesc};

mod reflect;
mod tokens;

fn main() {
    let crate_dir = PathBuf::from_str(&env::args().skip(1).next().unwrap()).unwrap();

    let dir = crate_dir.join("src").join("slang");

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

    let mut mod_code = quote! {
        #![allow(unused_imports)]

        use ash::vk::DescriptorSetLayoutBinding;
        use ash::vk::DescriptorType;
        use ash::vk::Format;
        use ash::vk::ShaderStageFlags;
        use ash::vk::VertexInputAttributeDescription;
        use ash::vk::VertexInputBindingDescription;
        use ash::vk::VertexInputRate;
        use slang_builder_runtime::CompiledShader;
        use std::marker::PhantomData;
    };

    _ = fs::create_dir(&bin_dir);

    for file in fs::read_dir(dir.join("src"))
        .unwrap()
        .map(|file| file.unwrap().path())
        .filter(|file| file.extension().map_or(false, |ext| ext == "slang"))
    {
        let content = fs::read_to_string(file.clone()).unwrap();

        let file_stem = file.file_stem().unwrap().to_str().unwrap();

        let module = session
            .load_module_from_source_string(file_stem, file.to_str().unwrap(), &content)
            .unwrap();

        let entry_point = module.find_entry_point_by_name("main").unwrap();

        let component = session
            .create_composite_component_type(&[module.into(), entry_point.into()])
            .unwrap();

        let linked_component = component.link().unwrap();

        let blob = linked_component.entry_point_code(0, 0).unwrap();

        let code = blob.as_slice();

        let shader_mod_code = reflect::reflect(code, content, file_stem);

        mod_code.extend(shader_mod_code);

        let bin_file = bin_dir.join(file_stem).with_added_extension("spv");

        fs::write(&bin_file, code).unwrap();

        old_files.retain(|old_file| old_file != &bin_file);
    }

    old_files
        .iter()
        .for_each(|file| fs::remove_file(file).unwrap());

    fs::write(dir.join("mod.rs"), mod_code.to_string()).unwrap();
}
