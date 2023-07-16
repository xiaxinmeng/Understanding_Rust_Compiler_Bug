sh
$ cd ~/Library/Logs/DiagnosticReports
$ sw_vers
ProductName:	Mac OS X
ProductVersion:	10.12.6
BuildVersion:	16G1036

$ clang --version
Apple LLVM version 9.0.0 (clang-900.0.38)
Target: x86_64-apple-darwin16.7.0
Thread model: posix
InstalledDir: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin

$ rustup show
Default host: x86_64-apple-darwin

installed toolchains
--------------------

stable-x86_64-apple-darwin
nightly-x86_64-apple-darwin (default)

active toolchain
----------------

nightly-x86_64-apple-darwin (default)
rustc 1.23.0-nightly (bd0e45a32 2017-11-06)

$ rustc --version
rustc 1.23.0-nightly (bd0e45a32 2017-11-06)

$ ll
total 112
-rw-------@ 1 huangxudong  staff    27K Nov  8 15:36 llvm-dsymutil_2017-11-08-153625_huangxudongs-Mac-Book-Pro.crash
-rw-------@ 1 huangxudong  staff    27K Nov  8 15:36 llvm-dsymutil_2017-11-08-153627_huangxudongs-Mac-Book-Pro.crash
