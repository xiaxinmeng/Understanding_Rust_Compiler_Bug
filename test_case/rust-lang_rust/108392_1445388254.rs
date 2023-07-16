
export CI=true
export JUST_FOR_RELEASE=true
export CARGO_BUILD_TARGET=armv7-unknown-linux-gnueabihf
export GLIBC_VERSION=2.17
export JUST_USE_CARGO_ZIGBUILD=true
just build
