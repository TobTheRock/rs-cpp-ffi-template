

# Shim library was already built with Rust
if(NOT BUILD_SHIM)
message("## Importing prebuilt C++ shim")

add_library(deep_thought_shim STATIC IMPORTED GLOBAL)

# Note: for release the path will differ
set_target_properties(deep_thought_shim PROPERTIES IMPORTED_LOCATION ${PROJECT_SOURCE_DIR}/target/debug/librs_cpp_ffi_template.a)
# set_target_properties(deep_thought_shim PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/include)

target_include_directories(deep_thought_shim INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)

else()

message("### Building and packaging C++ shim with Corrosion")
add_library(deep_thought_shim STATIC ${CMAKE_CURRENT_SOURCE_DIR}/src/deep_thought.cpp)
target_include_directories(deep_thought_shim PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Download corrosion from the github repo
# Alternatively one can install corrosion locally
include(FetchContent)
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
target_include_directories(deep_thought_ffi PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

target_link_libraries(deep_thought_shim PUBLIC deep_thought_ffi)


# only needed since corrosion renames the generated headers and we have to differentiate
target_compile_options(deep_thought_shim PRIVATE -DWITH_CORROSION)

endif()