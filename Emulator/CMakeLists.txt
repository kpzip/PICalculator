cmake_minimum_required(VERSION 3.10)
project(pic16emulator VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_MODULES ON)

file(GLOB_RECURSE SOURCES "src/*.cpp")
add_executable(pic16emulator ${SOURCES})
target_compile_options(pic16emulator PRIVATE -g)
