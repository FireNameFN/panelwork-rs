use std::{env, fs, path::Path};

use quote::quote;

fn main() {
    let mut files = fs::read_dir("resources/textures")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    files.sort_by_cached_key(|file| file.file_name());

    let textures = files.iter().map(|file| fs::read(file.path()).unwrap());

    let tokens = quote!(&[#(&[#(#textures),*]),*]);

    fs::write(
        Path::new(&env::var_os("OUT_DIR").unwrap()).join("textures.rs"),
        tokens.to_string(),
    )
    .unwrap();
}
