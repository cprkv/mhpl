## dependency status

- openal-soft: 1.20.1
  - forced usage of sdl2 library
  - forced static lib generation (no shared)
  - disabled `FIND_PACKAGE(SDL2)`
  - added `SDL2-static` after `EXPORT(TARGETS OpenAL `
- SDL2: 2.0.12
  - forced static lib generation (no shared)
  - removed `PROPERTIES DEBUG_POSTFIX`
- freetype: 2.10.2
- SDL2_ttf: 2.0.15
- vorbis: 1.3.6
  - removed `# Find ogg dependency....`
  - added `set(OGG_LIBRARIES ogg)` before `add_subdirectory(lib)`
- libogg: 1.3.2
  - removed `if(BUILD_TESTING)...endif()`
- angelscript: 2.19.2
  - created cmakelists from newer version
- CgToolkit: 3.1 - April 2012 (3.1.0013)