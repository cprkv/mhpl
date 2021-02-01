cmake_minimum_required(VERSION 3.4)
project(binreloc)

add_library(binreloc STATIC binreloc.c)
target_include_directories(binreloc PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_definitions(binreloc PRIVATE "-DENABLE_BINRELOC")
