
$ rustc --version
rustc 1.30.0-nightly (aaa170beb 2018-08-31)

$(rustc --print sysroot)/lib/rustlib/x86_64-apple-darwin/bin/rust-lld -flavor gnu -version
dyld: Library not loaded: @rpath/libLLVM.dylib
  Referenced from: /Users/parasyte/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/bin/rust-lld
  Reason: image not found
Abort trap: 6

$ sw_vers
ProductName:	Mac OS X
ProductVersion:	10.13.6
BuildVersion:	17G65
