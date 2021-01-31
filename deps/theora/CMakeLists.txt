cmake_minimum_required(VERSION 3.4)
project(theora)

# todo arm asm check
# todo asm x86_vc

set(THEORA_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/bitpack.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/decapiwrapper.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/decinfo.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/decode.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/dequant.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/huffdec.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/info.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/apiwrapper.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/fragment.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/idct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/internal.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/state.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/quant.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/analyze.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/encfrag.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/encapiwrapper.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/encinfo.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/encode.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/enquant.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/fdct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/huffenc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/mathops.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/mcenc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/rate.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/tokenize.c)

set(THEORA_X86_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/mmxencfrag.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/mmxfdct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/mmxfrag.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/mmxidct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/mmxstate.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/sse2encfrag.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/sse2fdct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/sse2idct.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/x86cpu.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/x86enc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/x86enquant.c
    ${CMAKE_CURRENT_SOURCE_DIR}/lib/x86/x86state.c)

check_cxx_source_compiles(
    "int main(int argc, char **argv) {
        #if !defined(__i386__)
            #error not an x86 host: preprocessor macro __i386__ not defined
        #endif
        return 0;
    }"
    THEORA_HAS_I386_TARGET
)
check_cxx_source_compiles(
    "int main(int argc, char **argv) {
        #if !defined(__x86_64__)
            #error not an x86_64 host: preprocessor macro __x86_64__ not defined
        #endif
        return 0;
    }"
    THEORA_HAS_X86_64_TARGET
)

if(THEORA_HAS_I386_TARGET)
    set(THEORA_COMPILE_FLAGS "-DOC_X86_ASM")
    set(THEORA_SOURCES ${THEORA_SOURCES} ${THEORA_X86_SOURCES})
elseif(THEORA_HAS_X86_64_TARGET)
    set(THEORA_COMPILE_FLAGS "-DOC_X86_ASM -DOC_X86_64_ASM")
    set(THEORA_SOURCES ${THEORA_SOURCES} ${THEORA_X86_SOURCES})
endif()

add_library(theora STATIC ${THEORA_SOURCES})
target_include_directories(theora PUBLIC include)
target_link_libraries(theora PUBLIC ogg)
target_compile_definitions(theora PRIVATE ${THEORA_COMPILE_FLAGS})
