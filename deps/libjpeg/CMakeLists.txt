cmake_minimum_required(VERSION 3.4)
project(jpeg C)

# source files: JPEG library proper
set(SOURCES
  source/jcapimin.c
  source/jcapistd.c
  source/jccoefct.c
  source/jccolor.c
  source/jcdctmgr.c
  source/jchuff.c
  source/jcinit.c
  source/jcmainct.c
  source/jcmarker.c
  source/jcmaster.c
  source/jcomapi.c
  source/jcparam.c
  source/jcphuff.c
  source/jcprepct.c
  source/jcsample.c
  source/jctrans.c
  source/jdapimin.c
  source/jdapistd.c
  source/jdatadst.c
  source/jdatasrc.c
  source/jdcoefct.c
  source/jdcolor.c
  source/jddctmgr.c
  source/jdhuff.c
  source/jdinput.c
  source/jdmainct.c
  source/jdmarker.c
  source/jdmaster.c
  source/jdmerge.c
  source/jdphuff.c
  source/jdpostct.c
  source/jdsample.c
  source/jdtrans.c
  source/jerror.c
  source/jfdctflt.c
  source/jfdctfst.c
  source/jfdctint.c
  source/jidctflt.c
  source/jidctfst.c
  source/jidctint.c
  source/jidctred.c
  source/jquant1.c
  source/jquant2.c
  source/jutils.c
  source/jmemmgr.c

  # mem manager (malloc/free)
  source/jmemnobs.c
  #  source/jmemansi.c
  #  source/jmemname.c
  #  source/jmemdos.c
  #  source/jmemmac.c
  )

add_library(jpeg STATIC ${SOURCES})
target_include_directories(jpeg PUBLIC include)
