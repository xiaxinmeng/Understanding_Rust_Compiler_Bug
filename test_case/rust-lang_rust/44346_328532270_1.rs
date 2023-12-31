
$ tar --transform 's/^\.\/disabled\//.\//' -C /home/kallisti5/Code/rust/src/ci/docker -c . | tar t
./
./README.md
./arm-android/
./arm-android/Dockerfile
./armhf-gnu/
./armhf-gnu/vexpress_config
./armhf-gnu/Dockerfile
./asmjs/
./asmjs/Dockerfile
./cross/
./cross/build-arm-musl.sh
./cross/build-rumprun.sh
./cross/install-mips-musl.sh
./cross/install-mipsel-musl.sh
./cross/install-x86_64-redox.sh
./cross/Dockerfile
./disabled/
./aarch64-gnu/
./aarch64-gnu/config
./aarch64-gnu/Dockerfile
./dist-aarch64-android/
./dist-aarch64-android/Dockerfile
./dist-armv7-android/
./dist-armv7-android/Dockerfile
./dist-i686-android/
./dist-i686-android/Dockerfile
./dist-x86_64-android/
./dist-x86_64-android/Dockerfile
./dist-x86_64-redox/
./dist-x86_64-redox/Dockerfile
./wasm32-exp/
./wasm32-exp/node.sh
./wasm32-exp/Dockerfile
./wasm32/
./wasm32/Dockerfile
./scripts/
./scripts/android-base-apt-get.sh
./scripts/android-ndk.sh
./scripts/android-sdk.sh
./scripts/android-start-emulator.sh
./scripts/cross-apt-packages.sh
./scripts/crosstool-ng.sh
./scripts/emscripten.sh
./scripts/emscripten-wasm.sh
./scripts/make3.sh
./scripts/qemu-bare-bones-addentropy.c
./scripts/qemu-bare-bones-rcS
./scripts/rustbuild-setup.sh
./scripts/sccache.sh
./.gitignore
./dist-aarch64-linux/
./dist-aarch64-linux/aarch64-linux-gnu.config
./dist-aarch64-linux/build-toolchains.sh
./dist-aarch64-linux/Dockerfile
./dist-android/
./dist-android/Dockerfile
./dist-arm-linux/
./dist-arm-linux/arm-linux-gnueabi.config
./dist-arm-linux/build-toolchains.sh
./dist-arm-linux/Dockerfile
./dist-armhf-linux/
./dist-armhf-linux/arm-linux-gnueabihf.config
./dist-armhf-linux/build-toolchains.sh
./dist-armhf-linux/Dockerfile
./dist-armv7-linux/
./dist-armv7-linux/armv7-linux-gnueabihf.config
./dist-armv7-linux/build-toolchains.sh
./dist-armv7-linux/Dockerfile
./dist-fuchsia/
./dist-fuchsia/build-toolchain.sh
./dist-fuchsia/compiler-rt-dso-handle.patch
./dist-fuchsia/shared.sh
./dist-fuchsia/Dockerfile
./dist-i586-gnu-i686-musl/
./dist-i586-gnu-i686-musl/build-musl.sh
./dist-i586-gnu-i686-musl/musl-libunwind-patch.patch
./dist-i586-gnu-i686-musl/Dockerfile
./dist-i686-freebsd/
./dist-i686-freebsd/build-toolchain.sh
./dist-i686-freebsd/Dockerfile
./dist-i686-linux/
./dist-i686-linux/build-binutils.sh
./dist-i686-linux/build-cmake.sh
./dist-i686-linux/build-curl.sh
./dist-i686-linux/build-gcc.sh
./dist-i686-linux/build-git.sh
./dist-i686-linux/build-headers.sh
./dist-i686-linux/build-openssl.sh
./dist-i686-linux/build-python.sh
./dist-i686-linux/shared.sh
./dist-i686-linux/Dockerfile
./dist-mips-linux/
./dist-mips-linux/Dockerfile
./dist-mips64-linux/
./dist-mips64-linux/Dockerfile
./dist-mips64el-linux/
./dist-mips64el-linux/Dockerfile
./dist-mipsel-linux/
./dist-mipsel-linux/Dockerfile
./dist-powerpc-linux/
./dist-powerpc-linux/build-powerpc-toolchain.sh
./dist-powerpc-linux/patches/
./dist-powerpc-linux/patches/glibc/
./dist-powerpc-linux/patches/glibc/2.12.2/
./dist-powerpc-linux/patches/glibc/2.12.2/001-PowerPC-Remove-unnecessary-mnew-mnemonics.patch
./dist-powerpc-linux/powerpc-linux-gnu.config
./dist-powerpc-linux/Dockerfile
./dist-powerpc64-linux/
./dist-powerpc64-linux/build-powerpc64-toolchain.sh
./dist-powerpc64-linux/patches/
./dist-powerpc64-linux/patches/glibc/
./dist-powerpc64-linux/patches/glibc/2.12.2/
./dist-powerpc64-linux/patches/glibc/2.12.2/001-PowerPC-Remove-unnecessary-mnew-mnemonics.patch
./dist-powerpc64-linux/patches/glibc/2.12.2/002-Prevent-inlining-in-PPC64-initfini.s.patch
./dist-powerpc64-linux/powerpc64-linux-gnu.config
./dist-powerpc64-linux/shared.sh
./dist-powerpc64-linux/Dockerfile
./dist-powerpc64le-linux/
./dist-powerpc64le-linux/build-powerpc64le-toolchain.sh
./dist-powerpc64le-linux/shared.sh
./dist-powerpc64le-linux/Dockerfile
./dist-s390x-linux/
./dist-s390x-linux/build-s390x-toolchain.sh
./dist-s390x-linux/patches/
./dist-s390x-linux/patches/glibc/
./dist-s390x-linux/patches/glibc/2.12.2/
./dist-s390x-linux/patches/glibc/2.12.2/001-Use-.machine-to-prevent-AS-from-complaining-about-z9.patch
./dist-s390x-linux/s390x-linux-gnu.config
./dist-s390x-linux/Dockerfile
./dist-x86_64-freebsd/
./dist-x86_64-freebsd/build-toolchain.sh
./dist-x86_64-freebsd/Dockerfile
./dist-x86_64-linux/
./dist-x86_64-linux/build-binutils.sh
./dist-x86_64-linux/build-cmake.sh
./dist-x86_64-linux/build-curl.sh
./dist-x86_64-linux/build-gcc.sh
./dist-x86_64-linux/build-git.sh
./dist-x86_64-linux/build-headers.sh
./dist-x86_64-linux/build-openssl.sh
./dist-x86_64-linux/build-python.sh
./dist-x86_64-linux/shared.sh
./dist-x86_64-linux/Dockerfile
./dist-x86_64-musl/
./dist-x86_64-musl/build-musl.sh
./dist-x86_64-musl/Dockerfile
./dist-x86_64-netbsd/
./dist-x86_64-netbsd/build-netbsd-toolchain.sh
./dist-x86_64-netbsd/Dockerfile
./i686-gnu-nopt/
./i686-gnu-nopt/Dockerfile
./i686-gnu/
./i686-gnu/Dockerfile
./scripts/
./scripts/android-base-apt-get.sh
./scripts/android-ndk.sh
./scripts/android-sdk.sh
./scripts/android-start-emulator.sh
./scripts/cross-apt-packages.sh
./scripts/crosstool-ng.sh
./scripts/emscripten-wasm.sh
./scripts/emscripten.sh
./scripts/make3.sh
./scripts/qemu-bare-bones-addentropy.c
./scripts/qemu-bare-bones-rcS
./scripts/rustbuild-setup.sh
./scripts/sccache.sh
./x86_64-gnu-aux/
./x86_64-gnu-aux/Dockerfile
./x86_64-gnu-debug/
./x86_64-gnu-debug/Dockerfile
./x86_64-gnu-distcheck/
./x86_64-gnu-distcheck/Dockerfile
./x86_64-gnu-full-bootstrap/
./x86_64-gnu-full-bootstrap/Dockerfile
./x86_64-gnu-incremental/
./x86_64-gnu-incremental/Dockerfile
./x86_64-gnu-llvm-3.7/
./x86_64-gnu-llvm-3.7/Dockerfile
./x86_64-gnu-nopt/
./x86_64-gnu-nopt/Dockerfile
./x86_64-gnu/
./x86_64-gnu/Dockerfile
./run.sh
