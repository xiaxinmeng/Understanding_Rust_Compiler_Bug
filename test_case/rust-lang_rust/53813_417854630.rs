
error: linking with `rust-lld` failed: signal: 6
  |
<stripped>
note: dyld: Library not loaded: @rpath/libLLVM.dylib
            Referenced from: /Users/pepyakin/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/bin/rust-lld
            Reason: image not found

$ find ~/.rustup/toolchains/nightly-x86_64-apple-darwin | grep -i libLLVM.dylib | wc -l
       0

$ rustc +nightly -V
rustc 1.30.0-nightly (aaa170beb 2018-08-31)

$ sw_vers
ProductName:	Mac OS X
ProductVersion:	10.13.3
BuildVersion:	17D102

