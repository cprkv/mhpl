Modern HPL
==========

Engine based on Amnesia: The Dark Descent Source Code.

## Todo

- [ ] replace -lpng via png in deps folder

## Install extra dependencies

on ubuntu:
```shell
sudo apt install libasound2-data libasound2-dev libasound2 libasound2-plugins libpulse-dev
```

## Build

```shell
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
cmake --build . --target Amnesia -j 10
```

## Run

`./path-to-Amnesia -cwd` - will run Amnesia from current directory as data directory.

## HPL2 notice
Currently the engine uses fbx sdk 2012 which isn't available anymore which means the engine wont compile. If you want to give a shot anyway you can find the sdk here:
https://www.autodesk.com/fbx

Other than that, here is almost everything you need to build Amnesia: The Dark Descent. Included are project files for Visual Studio 2010 and CMake for Linux & macOS. 

## License Information
All code is under the GPL Version 3 license. Read the LICENSE file for terms of use of the license.
