cmake_minimum_required(VERSION 3.10)
project(pic16emulator VERSION 0.1.0 LANGUAGES C CXX)

file(GLOB_RECURSE SOURCES "src/*.cpp")
add_executable(pic16emulator ${SOURCES})
target_compile_options(pic16emulator PRIVATE -g)
