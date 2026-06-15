use std::path::PathBuf;
use std::{env, fs};

use shader_slang::{CompileTarget, SessionDesc};
use shader_slang::{GlobalSession, TargetDesc};

fn main() {
    let dir = PathBuf::from_iter([env::args().skip(1).next().unwrap().as_str(), "shaders"]);

    let bin_dir = dir.join("bin");

    _ = fs::create_dir(&bin_dir);

    fs::read_dir(&bin_dir)
        .unwrap()
        .map(|file| file.unwrap().path())
        .filter(|file| file.extension().map_or(false, |ext| ext == "spv"))
        .for_each(|file| fs::remove_file(file).unwrap());

    let global_session = GlobalSession::new().unwrap();

    //let compiler_options = CompilerOptions::default();

    let target_desc = [TargetDesc::default()
        .format(CompileTarget::Spirv)
        .profile(global_session.find_profile("spirv_1_5"))];

    let session_desc = SessionDesc::default().targets(&target_desc);

    let session = global_session.create_session(&session_desc).unwrap();

    println!("{}", dir.join("src").to_str().unwrap());

    for file in fs::read_dir(dir.join("src")).unwrap() {
        let file_result = file.unwrap();

        let module = session
            .load_module(file_result.path().to_str().unwrap())
            .unwrap();

        let entry_point = module.find_entry_point_by_name("main").unwrap();

        let component = session
            .create_composite_component_type(&[module.into(), entry_point.into()])
            .unwrap();

        let linked_component = component.link().unwrap();

        let code = linked_component.entry_point_code(0, 0).unwrap();

        fs::write(
            bin_dir
                .join(file_result.file_name().to_str().unwrap())
                .with_extension("spv"),
            code.as_slice(),
        )
        .unwrap();
    }
}
