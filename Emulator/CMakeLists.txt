cmake_minimum_required(VERSION 3.10)
project(pic16emulator VERSION 0.1.0 LANGUAGES C)

add_executable(pic16emulator src/main.c src/pic16machine.c)
target_compile_options(pic16emulator PRIVATE -g)