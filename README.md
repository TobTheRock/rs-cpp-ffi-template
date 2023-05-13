# Template: C++ bindings with Rust

## The Cxx crate
Reference: https://cxx.rs/

### Build flow

## CLI example

### Shim library built with Rust 
Built the Rust library with:
```
cargo b --features=build_shim
```
this already builts (with `build.rs`) a static library including the sources generated by `cxx`  and as well those of the shim layer (`src/cpp/deep_thought.cpp`).

Thus we only need to link against this library and include its headers to build the CLI executable using `CMake` (see `cli/CMakeLists.txt`) with:
```
cmake -S . -B target/cmake
cmake --build target/cmake
```

### Shim library built with CMake/Corrosion 
https://github.com/corrosion-rs/corrosion

```
cmake -S . -B target/cmake -DBUILD_SHIM=1
cmake --build target/cmake
```

### Running the CLI
Just execute
```
./target/cmake/cli/deep_thought_cli
```
and prompt your question which `DeepThought™` will gladly answer.
