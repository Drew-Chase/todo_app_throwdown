cmake_minimum_required(VERSION 3.15)
project(todo_commonlib LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(rust_dir release)
    set(rust_flags --release)
else()
    set(rust_dir debug)
    set(rust_flags)
endif()
set(todo_commonlib_file ${CMAKE_SOURCE_DIR}/target/${rust_dir}/todo_commonlib.lib)
add_custom_target(rust_build
        COMMAND cargo build --package todo_commonlib ${rust_flags}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        BYPRODUCTS ${todo_commonlib_file}
)

add_library(todo_commonlib_rs STATIC IMPORTED)
set_target_properties(todo_commonlib_rs PROPERTIES
        IMPORTED_LOCATION ${todo_commonlib_file}
        INTERFACE_LINK_LIBRARIES "legacy_stdio_definitions;kernel32;ntdll;userenv;ws2_32;dbghelp"
)
add_library(todo_commonlib STATIC src/todo_commonlib.cpp)
target_include_directories(todo_commonlib PUBLIC include)
target_link_libraries(todo_commonlib PRIVATE todo_commonlib_rs)
add_dependencies(todo_commonlib rust_build)