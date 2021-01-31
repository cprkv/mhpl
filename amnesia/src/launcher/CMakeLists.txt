cmake_minimum_required (VERSION 2.8.11)
project(Launcher)

#FindPrebuiltLibrary(FLTK_LIBRARY fltk)
#FindPrebuiltLibrary(FLTK_IMAGES_LIBRARY fltk_images)
#FindPrebuiltLibrary(JPEG_LIBRARY jpeg)

add_executable(Launcher launcher.cpp Main.cpp QualityChooser.cpp LauncherHelper.cpp)

# for windows: opengl32.lib fltk_1_3_jpeg.lib fltk_1_3_images.lib fltk_1_3.lib Hpl2.lib wsock32.lib comctl32.lib zlibwapi.lib
# for linux: X11 Xext Xft fontconfig ${JPEG_LIBRARY}
target_link_libraries(Launcher HPL2 fltk_images fltk)

# vim: et ts=4
