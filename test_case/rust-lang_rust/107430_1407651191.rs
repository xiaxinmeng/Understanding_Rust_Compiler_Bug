
export CC=gcc CXX=g++
export MACH_USE_SYSTEM_PYTHON=1
export MOZBUILD_STATE_PATH=${PWD}/mozbuild
./mach configure
./mach build

