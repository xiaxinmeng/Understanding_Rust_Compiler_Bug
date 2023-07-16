yaml
    # OSX builders producing releases. These do not run the full test suite and
    # just produce a bunch of artifacts.
    #
    # Note that these are running in the `xcode7` image instead of the
    # `xcode8.2` image as above. That's because we want to build releases for
    # OSX 10.7 and `xcode7` is the latest Xcode able to compile LLVM for 10.7.
    - env: >
        RUST_CHECK_TARGET=dist
        RUST_CONFIGURE_ARGS="--build=i686-apple-darwin --enable-extended"
        SRC=.
        DEPLOY=1
        RUSTC_RETRY_LINKER_ON_SEGFAULT=1
        SCCACHE_ERROR_LOG=/tmp/sccache.log
        MACOSX_DEPLOYMENT_TARGET=10.7
      os: osx
      osx_image: xcode7
      install: *osx_install_sccache
    - env: >
        RUST_CHECK_TARGET=dist
        RUST_CONFIGURE_ARGS="--target=aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios --enable-extended"
        SRC=.
        DEPLOY=1
        RUSTC_RETRY_LINKER_ON_SEGFAULT=1
        SCCACHE_ERROR_LOG=/tmp/sccache.log
        MACOSX_DEPLOYMENT_TARGET=10.7
      os: osx
      osx_image: xcode7
      install: *osx_install_sccache
