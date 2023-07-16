
$ rustup default nightly-2017-12-03
info: using existing install for 'nightly-2017-12-03-x86_64-apple-darwin'
info: default toolchain set to 'nightly-2017-12-03-x86_64-apple-darwin'

  nightly-2017-12-03-x86_64-apple-darwin unchanged - rustc 1.24.0-nightly (f9b0897c5 2017-12-02)

$ rustc t.rs -l c
$ ./t
$ rustup default nightly-2017-12-04
info: using existing install for 'nightly-2017-12-04-x86_64-apple-darwin'
info: default toolchain set to 'nightly-2017-12-04-x86_64-apple-darwin'

  nightly-2017-12-04-x86_64-apple-darwin unchanged - rustc 1.24.0-nightly (1956d5535 2017-12-03)

$ rustc t.rs -l c
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "<RUSTUP>/toolchains/nightly-2017-12-04-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "t.t0.rcgu.o" "t.t1.rcgu.o" "-o" "t" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "<RUSTUP>/toolchains/nightly-2017-12-04-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "-l" "c" "<RUSTUP>/toolchains/nightly-2017-12-04-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcore-23bf36c939af2dde.rlib"
  = note: Undefined symbols for architecture x86_64:
            "_rust_eh_personality", referenced from:
                Dwarf Exception Unwind Info (__eh_frame) in t.t1.rcgu.o
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)


error: aborting due to previous error
