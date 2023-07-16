
# export TOOLCHAIN=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64
export TOOLCHAIN=~/.NDK/arm
#export TARGET=aarch64-linux-android
export TARGET=arm-linux-androideabi
# export TARGET=i686-linux-android
#export TARGET=x86_64-linux-android
# Set this to your minSdkVersion.
# export API=26
# Configure and build.
export AR=$TOOLCHAIN/bin/$TARGET-ar
export AS=$TOOLCHAIN/bin/$TARGET-as
export CC=$TOOLCHAIN/bin/$TARGET-clang
export CXX=$TOOLCHAIN/bin/$TARGET-clang++
export LD=$TOOLCHAIN/bin/$TARGET-ld
export RANLIB=$TOOLCHAIN/bin/$TARGET-ranlib
export STRIP=$TOOLCHAIN/bin/$TARGET-strip

export TARGET_CC=$CC
export TARGET_CXX=$CXX
# export TARGET_CFLAGS=$CFLAGS
# export TARGET_CXXFLAGS=$CXXFLAGS

# export CFLAGS='-D__ANDROID_API__=26'
# export CFLAGS='-march=armv7'
# export TARGET_CFLAGS='-D__ANDROID_API__=26'

# export CFLAGS='-march=armv7-a'

PKG_CONFIG_ALLOW_CROSS=1 RUST_BACKTRACE=full OPENSSL_STATIC=1  cargo build --target armv7-linux-androideabi --release

