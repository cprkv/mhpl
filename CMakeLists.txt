cmake_minimum_required(VERSION 3.6)
project(mhpl)

option(ALSOFT_REQUIRE_SDL2 "Require SDL2 backend" ON)
option(ALSOFT_BACKEND_SDL2 "Enable SDL2 backend" ON)

add_subdirectory(deps/zlib)
add_subdirectory(deps/freetype)
add_subdirectory(deps/SDL2)
add_subdirectory(deps/SDL2_ttf)
add_subdirectory(deps/openal-soft)
add_subdirectory(deps/libogg)
add_subdirectory(deps/libjpeg)
add_subdirectory(deps/vorbis)
add_subdirectory(deps/OALWrapper)
add_subdirectory(deps/angelscript)
add_subdirectory(deps/CgToolkit)
add_subdirectory(deps/newton-dynamics)
add_subdirectory(deps/glew/build/cmake)
add_subdirectory(deps/theora)
add_subdirectory(deps/DevIL)
add_subdirectory(deps/fltk)

add_subdirectory(HPL2/core)
add_subdirectory(amnesia/src)
#add_subdirectory(HPL2/tests tests)
add_subdirectory(HPL2/tools tools)
