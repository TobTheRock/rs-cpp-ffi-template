add_executable(deep_thought_cli main.cpp)

target_link_libraries(deep_thought_cli PUBLIC deep_thought_shim)
