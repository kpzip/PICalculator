cmake_minimum_required(VERSION 3.10)
project(simplecalculator VERSION 0.1.0 LANGUAGES C)

file(GLOB_RECURSE SOURCES "src/*.c")
add_executable(pic16emulator ${SOURCES})
target_compile_options(pic16emulator PRIVATE -g)
