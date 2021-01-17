cmake_minimum_required(VERSION 3.5)

project(angelscript)

find_package(Threads)

set(ANGELSCRIPT_HEADERS
        angelscript/include/angelscript.h
        angelscript/source/as_compiler.h
        angelscript/source/as_scriptfunction.h
        angelscript/source/as_memory.h
        angelscript/source/as_configgroup.h
        angelscript/source/as_thread.h
        angelscript/source/as_module.h
        angelscript/source/as_variablescope.h
        angelscript/source/as_scriptnode.h
        angelscript/source/as_callfunc.h
        angelscript/source/as_restore.h
        angelscript/source/as_datatype.h
        angelscript/source/as_typeinfo.h
        angelscript/source/as_objecttype.h
        angelscript/source/as_property.h
        angelscript/source/as_context.h
        angelscript/source/as_scriptcode.h
        angelscript/source/as_debug.h
        angelscript/source/as_scriptobject.h
        angelscript/source/as_gc.h
        angelscript/source/as_outputbuffer.h
        angelscript/source/as_atomic.h
        angelscript/source/as_scriptengine.h
        angelscript/source/as_builder.h
        angelscript/source/as_map.h
        angelscript/source/as_arrayobject.h
        angelscript/source/as_tokendef.h
        angelscript/source/as_config.h
        angelscript/source/as_string_util.h
        angelscript/source/as_tokenizer.h
        angelscript/source/as_array.h
        angelscript/source/as_criticalsection.h
        angelscript/source/as_texts.h
        angelscript/source/as_generic.h
        angelscript/source/as_string.h
        angelscript/source/as_bytecode.h
        angelscript/source/as_parser.h
        )

set(ANGELSCRIPT_SOURCE
        angelscript/source/as_configgroup.cpp
        angelscript/source/as_scriptcode.cpp
        angelscript/source/as_module.cpp
        angelscript/source/as_memory.cpp
        angelscript/source/as_string_util.cpp
        angelscript/source/as_tokenizer.cpp
        angelscript/source/as_thread.cpp
        angelscript/source/as_typeinfo.cpp
        angelscript/source/as_callfunc_mips.cpp
        angelscript/source/as_arrayobject.cpp
        angelscript/source/as_generic.cpp
        angelscript/source/as_callfunc_xenon.cpp
        angelscript/source/as_callfunc_ppc_64.cpp
        angelscript/source/as_gc.cpp
        angelscript/source/as_callfunc_ppc.cpp
        angelscript/source/as_callfunc_arm.cpp
        angelscript/source/as_globalproperty.cpp
        angelscript/source/as_string.cpp
        angelscript/source/as_outputbuffer.cpp
        angelscript/source/as_variablescope.cpp
        angelscript/source/as_builder.cpp
        angelscript/source/as_context.cpp
        angelscript/source/as_compiler.cpp
        angelscript/source/as_parser.cpp
        angelscript/source/as_objecttype.cpp
        angelscript/source/as_scriptengine.cpp
        angelscript/source/as_scriptfunction.cpp
        angelscript/source/as_scriptobject.cpp
        angelscript/source/as_restore.cpp
        angelscript/source/as_callfunc_sh4.cpp
        angelscript/source/as_callfunc.cpp
        angelscript/source/as_atomic.cpp
        angelscript/source/as_scriptnode.cpp
        angelscript/source/as_bytecode.cpp
        angelscript/source/as_datatype.cpp
        )

if (MSVC AND CMAKE_CL_64)
    enable_language(ASM_MASM)
    if (CMAKE_ASM_MASM_COMPILER_WORKS)
        set(ANGELSCRIPT_SOURCE ${ANGELSCRIPT_SOURCE} angelscript/source/as_callfunc_x64_msvc_asm.asm)
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