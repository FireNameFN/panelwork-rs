fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    println!("cargo::rustc-link-arg=-Wl,-rpath,{}/deps/slang/lib", dir);
}
