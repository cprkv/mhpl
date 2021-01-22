cmake_minimum_required(VERSION 3.4)
project(OALWrapper)

set(OALW_SOURCES
  sources/OAL_AudioData.cpp
  sources/OAL_Buffer.cpp
  sources/OAL_Device.cpp
  sources/OAL_Effect.cpp
  sources/OAL_Effect_Reverb.cpp
  sources/OAL_EffectSlot.cpp
  sources/OAL_EFX.cpp
  sources/OAL_EFXManager.cpp
  sources/OAL_Filter.cpp
  sources/OAL_Helper.cpp
  sources/OAL_Init.cpp
  sources/OAL_Loaders.cpp
  sources/OAL_LoggerObject.cpp
  sources/OAL_OggSample.cpp
  sources/OAL_OggStream.cpp
  sources/OAL_WAVSample.cpp
  sources/OAL_CustomStream.cpp
  sources/OAL_Playback.cpp
  sources/OAL_Sample.cpp
  sources/OAL_Source.cpp
  sources/OAL_SourceManager.cpp
  sources/OAL_Stream.cpp
  sources/OAL_Types.cpp)

add_library(OALWrapper STATIC ${OALW_SOURCES})
target_link_libraries(OALWrapper PUBLIC vorbisfile vorbis ogg SDL2-static OpenAL)
target_include_directories(OALWrapper PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

add_executable(OALWSimpleTest tests/Simple/main.cpp)
target_link_libraries(OALWSimpleTest OALWrapper)

add_executable(OALWPlaySound tests/PlaySound/main.cpp)
target_link_libraries(OALWPlaySound OALWrapper)

# not compiles \_@-@_/
# add_executable(OALWPlaylist tests/Playlist/main.cpp)
# target_link_libraries(OALWPlaylist OALWrapper)

add_executable(OALWCustomStream tests/CustomStream/main.cpp)
target_link_libraries(OALWCustomStream OALWrapper)
