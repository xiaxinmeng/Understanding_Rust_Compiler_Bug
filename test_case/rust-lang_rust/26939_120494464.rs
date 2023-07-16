
$ git checkout stable
$ git submodule update --init --recursive
$ cd build && --target=armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,aarch64-apple-ios,x86_64-apple-ios --prefix=/opt/rust-ios-toolchains && make install
