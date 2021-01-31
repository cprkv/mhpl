cmake_minimum_required (VERSION 2.8.11)
project(Lux)

set(EXECUTABLE_OUTPUT_PATH ${CMAKE_CURRENT_BINARY_DIR})

OPTION(USERDIR_RESOURCES "Allow using local userdirectory resources" ON)

set(MACOSX_BUNDLE_COPYRIGHT "Copyright © 2010-2013 Frictional Games")

# OPTION(IS_APPLE_64 "Apple build in 64 bits" OFF)
# TODO this is BAAAD
#if(APPLE)
#    if(NOT IS_APPLE_64)
#        set(CMAKE_OSX_ARCHITECTURES "i386")
#    endif()
#endif()

# TODO this is BAAAD
# silence some annoying warnings
if (CMAKE_CXX_COMPILER_ID MATCHES ".*clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    add_definitions(
        -Wno-switch
        -Wno-reorder
        -Wno-unused-variable
        -Wno-unused-function
    )
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
    add_definitions(
        -D_CRT_SECURE_NO_WARNINGS
    )
endif()

if(USERDIR_RESOURCES)
    add_definitions(-DUSERDIR_RESOURCES)
endif()

set(VERSION "1.3.1")

add_subdirectory(game game)

if(APPLE)
    add_subdirectory(launcher-macosx launcher)
else()
    add_subdirectory(launcher launcher)
endif()

add_custom_target(GameRelease DEPENDS Amnesia Launcher)
