cmake_minimum_required(VERSION 3.4)

add_library(cg SHARED IMPORTED)
set_target_properties(cg PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/bin/cg)

add_library(cgGL SHARED IMPORTED)
set_target_properties(cgGL PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/bin/cgGL)

add_library(CgToolkit INTERFACE)
set_property(TARGET CgToolkit PROPERTY INTERFACE_LINK_LIBRARIES cg cgGL)
target_include_directories(CgToolkit INTERFACE $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>)
