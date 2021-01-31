cmake_minimum_required(VERSION 3.5)

project(angelscript)

find_package(Threads)

set(ANGELSCRIPT_HEADERS
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/include/angelscript.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_compiler.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptfunction.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_memory.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_configgroup.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_thread.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_module.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_variablescope.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptnode.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_restore.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_datatype.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_typeinfo.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_objecttype.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_property.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_context.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptcode.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_debug.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptobject.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_gc.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_outputbuffer.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_atomic.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptengine.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_builder.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_map.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_arrayobject.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_tokendef.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_config.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_string_util.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_tokenizer.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_array.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_criticalsection.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_texts.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_generic.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_string.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_bytecode.h
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_parser.h
        )

set(ANGELSCRIPT_SOURCE
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_configgroup.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptcode.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_module.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_memory.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_string_util.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_tokenizer.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_thread.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_typeinfo.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_mips.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_arrayobject.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_generic.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_xenon.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_ppc_64.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_gc.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_ppc.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_arm.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_globalproperty.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_string.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_outputbuffer.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_variablescope.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_builder.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_context.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_compiler.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_parser.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_objecttype.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptengine.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptfunction.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptobject.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_restore.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_sh4.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_atomic.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_scriptnode.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_bytecode.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_datatype.cpp
        )

if (MSVC AND CMAKE_CL_64)
    enable_language(ASM_MASM)
    if (CMAKE_ASM_MASM_COMPILER_WORKS)
        set(ANGELSCRIPT_SOURCE ${ANGELSCRIPT_SOURCE}
            ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_x64_msvc.asm
            ${CMAKE_CURRENT_SOURCE_DIR}/angelscript/source/as_callfunc_x64_msvc.cpp)
    else ()
        message(FATAL ERROR "MSVC x86_64 target requires a working assembler")
    endif ()
endif ()

if (${CMAKE_SYSTEM_PROCESSOR} MATCHES "^arm")
    enable_language(ASM)
    if (CMAKE_ASM_COMPILER_WORKS)
        set(ANGELSCRIPT_SOURCE ${ANGELSCRIPT_SOURCE} angelscript/source/as_callfunc_arm.cpp angelscript/source/as_callfunc_arm_gcc.S)
        # todo what is this?
        #        set_property(SOURCE angelscript/source/as_callfunc_arm_gcc.S APPEND PROPERTY COMPILE_FLAGS " -Wa,-mimplicit-it=always")
    else ()
        message(FATAL ERROR "ARM target requires a working assembler")
    endif ()
endif ()

# todo:
#angelscript/source/as_callfunc_x86.cpp
#angelscript/source/as_callfunc_x64_gcc.cpp

if (MSVC)
    set(CMAKE_DEBUG_POSTFIX "d")
endif ()

set(ANGELSCRIPT_LIBRARY_NAME angelscript)
set(ANGELSCRIPT_LIBRARY_NAME ${ANGELSCRIPT_LIBRARY_NAME} CACHE STRING "" FORCE)

add_library(${ANGELSCRIPT_LIBRARY_NAME} ${ANGELSCRIPT_SOURCE} ${ANGELSCRIPT_HEADERS})

set_target_properties(${ANGELSCRIPT_LIBRARY_NAME} PROPERTIES
        CXX_STANDARD 20
        CXX_STANDARD_REQUIRED TRUE)

get_filename_component(ANGELSCRIPT_ABSOLUTE_INClUDE_DIR
        "${CMAKE_CURRENT_SOURCE_DIR}/angelscript/include"
        ABSOLUTE)

target_include_directories(${ANGELSCRIPT_LIBRARY_NAME} PUBLIC $<BUILD_INTERFACE:${ANGELSCRIPT_ABSOLUTE_INClUDE_DIR}>)

if (MSVC)
    target_compile_definitions(${ANGELSCRIPT_LIBRARY_NAME} PRIVATE -D_CRT_SECURE_NO_WARNINGS)
endif ()

target_compile_definitions(${ANGELSCRIPT_LIBRARY_NAME} PRIVATE -DANGELSCRIPT_EXPORT -D_LIB)

if (AS_NO_EXCEPTIONS)
    target_compile_definitions(${ANGELSCRIPT_LIBRARY_NAME} PRIVATE AS_NO_EXCEPTIONS)
endif ()

# Fix x64 issues on Linux
if ("${CMAKE_SYSTEM_PROCESSOR}" STREQUAL "x86_64" AND UNIX AND NOT APPLE)
    target_compile_options(${ANGELSCRIPT_LIBRARY_NAME} PRIVATE -fPIC)
endif ()

# Don't override the default library output path to avoid conflicts when building for multiple target platforms
#set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/../../lib)
target_link_libraries(${ANGELSCRIPT_LIBRARY_NAME} Threads::Threads)

if (BUILD_FRAMEWORK)
    set_target_properties(${ANGELSCRIPT_LIBRARY_NAME} PROPERTIES
            FRAMEWORK TRUE
            FRAMEWORK_VERSION ${PROJECT_VERSION}
            MACOSX_FRAMEWORK_IDENTIFIER com.angelcode.Angelscript
            MACOSX_FRAMEWORK_SHORT_VERSION_STRING ${PROJECT_VERSION}
            MACOSX_FRAMEWORK_BUNDLE_VERSION ${PROJECT_VERSION}
            XCODE_ATTRIBUTE_INSTALL_PATH "@rpath"
            PUBLIC_HEADER ../../include/angelscript.h
            )
endif ()

if (MSVC)
    set_target_properties(${ANGELSCRIPT_LIBRARY_NAME} PROPERTIES COMPILE_FLAGS "/MP")
endif ()

include(CMakePackageConfigHelpers)

set(ConfigPackageLocation lib/cmake/Angelscript)