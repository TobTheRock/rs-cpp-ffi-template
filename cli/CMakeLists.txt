cmake_minimum_required(VERSION 3.0.0)
project(cpp-example VERSION 0.1.0)

add_executable(deep_thought_cli main.cpp)

# In this case you would usually package the library accordingly
target_include_directories(deep_thought_cli PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include/)
target_link_directories(deep_thought_cli PRIVATE ../target/debug)
target_link_libraries(deep_thought_cli PUBLIC librs_cpp_ffi_template.a)


set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
