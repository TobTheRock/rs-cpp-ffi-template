

# Shim library was already built with Rust
if(BUILD_SHIM)
message("## Importing prebuilt C++ shim")

add_library(deep_thought_shim STATIC IMPORTED)

# Note: for release the path will differ
set_target_properties(deep_thought_shim PROPERTIES IMPORTED_LOCATION ../target/debug/librs_cpp_ffi_template.a)

target_include_directories(deep_thought_shim INTERFACE include)

else()

message("### Building and packaging C++ shim with Corrosion")
add_library(deep_thought_shim STATIC)

include(FetchContent)

# Alternatively one can install corrosion locally
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    # we still need the unstable API
    # GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH ../Cargo.toml)

# currently unstable
corrosion_add_cxxbridge(deep_thought_ffi
        CRATE rs-cpp-ffi-template
        FILES bindings.rs
)
target_link_libraries(deep_thought_shim PUBLIC deep_thought_ffi)

# add shim source
target_sources(deep_thought_shim PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src/deep_thought.cpp)
target_include_directories(deep_thought_shim PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

endif()