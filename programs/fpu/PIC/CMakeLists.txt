cmake_minimum_required(VERSION 3.10)
project(picpassthrough VERSION 0.1.0 LANGUAGES C)

file(GLOB_RECURSE SOURCES "src/*.c")
add_executable(picpassthrough ${SOURCES})
target_compile_options(picpassthrough PRIVATE -g)
