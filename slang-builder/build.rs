fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    //println!("cargo::rustc-link-search=deps/slang/lib");
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}/deps/slang/lib", dir);
    //println!("cargo::rustc-env=SLANG_DIR=/home/fn/Works/Rust/panelwork/slang-builder/deps/slang")
}
