
llvm[5]: ======= Finished Linking Release+Asserts Executable Sample (without symbols)
make[2]: Nothing to be done for `all'.
llvm[1]: ***** Completed Release+Asserts Build
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/_context.o
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/record_sp.o
compile: x86_64-apple-darwin/rt/stage0/rust_try.o
link: x86_64-apple-darwin/rt/stage0/librustrt.a
/opt/local/bin/ranlib: file: x86_64-apple-darwin/rt/stage0/librustrt.a(rust_android_dummy.o) has no symbols
/opt/local/bin/ranlib: file: x86_64-apple-darwin/rt/stage0/librustrt.a(record_sp.o) has no symbols
cp: x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/librustrt.a
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/morestack.o
link: x86_64-apple-darwin/rt/stage0/arch/x86_64/libmorestack.a
cp: x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/libmorestack.a
compile_and_link: x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/libstd.dylib
error: ar s x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/libstd-3e5aeb83-0.9.rlib failed with: exit code: 1
note: stdout ---

note: stderr ---
Assertion failed: ((unsigned)Val < Attribute::EndAttrKinds && "Attribute out of range!"), function addAttribute, file Attributes.cpp, line 948.
ar: fatal error in /opt/local/bin/ranlib

error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/snap3-mac/build/src/libsyntax/diagnostic.rs:75
task '<main>' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/snap3-mac/build/src/librustc/lib.rs:453
make: *** [x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/libstd.dylib] Error 101
