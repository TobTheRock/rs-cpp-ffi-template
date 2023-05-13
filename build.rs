#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    // rerun build when cpp
    println!("cargo:rerun-if-changed=src/cpp/deep_thought.cpp");

    // cc compiler, where
    // - cxx_build will first generate some C++ bindings
    // - adds target/cxxbridge to the includes
    let mut cc = cxx_build::bridge("src/bindings.rs");

    #[cfg(feature = "build_shim")]
    build_shim(&mut cc);
}

fn build_shim(cc: &mut cc::Build) {
    // Add the public includes
    let pub_include_dir = std::env::current_dir()
        .unwrap()
        .as_path()
        .join("./shim/include");
    cxx_build::CFG.exported_header_dirs.push(&pub_include_dir);

    cc.file("shim/src/deep_thought.cpp") // c++ source files which should be build
        .include("shim/include")
        .compile("cpp_shim");
}
