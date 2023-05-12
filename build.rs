fn main() {
    // rerun build when cpp source where cnahge
    println!("cargo:rerun-if-changed=src/cpp/deep_thought.cpp");

    // cc compiler, where
    // - cxx_build will first generate some C++ bindings
    // - adds target/cxxbridge to the includes
    let mut cc = cxx_build::bridge("src/bindings.rs");

    // TODO feature flag
    // #[cfg(feature = "build-with-rust")]
    // Add the public includes
    let pub_include_dir = std::env::current_dir().unwrap().as_path().join("./include");
    cxx_build::CFG.exported_header_dirs.push(&pub_include_dir);

    cc.file("src/cpp/deep_thought.cpp") // c++ source files which should be build
        .compile("deep_thought");
}
