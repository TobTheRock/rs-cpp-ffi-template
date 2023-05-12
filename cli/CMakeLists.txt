cmake_minimum_required(VERSION 3.0.0)
project(cpp-example VERSION 0.1.0)

add_executable(deep_thought_cli main.cpp)

# Shim library was already built with Rust
if(NOT USE_CORROSION)

message("Building CLI with prepackaged library")

# In this case you would usually package the library accordingly
target_include_directories(deep_thought_cli PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include/)
target_link_directories(deep_thought_cli PRIVATE ../target/debug)
target_link_libraries(deep_thought_cli PUBLIC librs_cpp_ffi_template.a)


set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

else()
# TODO move to subdirectory?

message("Building CLI with Corrosion")

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
target_link_libraries(deep_thought_cli PUBLIC deep_thought_ffi)

# add shim source
target_sources(deep_thought_cli PUBLIC ../src/cpp/deep_thought.cpp)
target_include_directories(deep_thought_cli PUBLIC ../include)

endif()