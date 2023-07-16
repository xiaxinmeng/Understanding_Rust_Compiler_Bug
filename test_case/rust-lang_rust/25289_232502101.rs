
> rustc --version
rustc 1.10.0 (cfcb716cf 2016-07-03)
> cargo --version
cargo 0.11.0-nightly (259324c 2016-05-20)
> rustup show
Default host: x86_64-apple-darwin

installed toolchains
--------------------

stable-x86_64-apple-darwin (default)
1.8.0-x86_64-apple-darwin

installed targets for active toolchain
--------------------------------------

mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
x86_64-apple-darwin

active toolchain
----------------

stable-x86_64-apple-darwin (default)
rustc 1.10.0 (cfcb716cf 2016-07-03)
> cargo new hello --bin && cd hello
> cargo build --target=mipsel-unknown-linux-gnu
   Compiling hello v0.1.0 (file:///Users/Jon/Code/hello)
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib" "/Users/Jon/Code/hello/target/mipsel-unknown-linux-gnu/debug/hello.0.o" "-o" "/Users/Jon/Code/hello/target/mipsel-unknown-linux-gnu/debug/hello" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/Users/Jon/Code/hello/target/mipsel-unknown-linux-gnu/debug" "-L" "/Users/Jon/Code/hello/target/mipsel-unknown-linux-gnu/debug/deps" "-L" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/libstd-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/libcollections-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/libpanic_unwind-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/librustc_unicode-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/libunwind-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/librand-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/liballoc-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/liballoc_jemalloc-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/liblibc-e8edd0fd.rlib" "/Users/Jon/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/mipsel-unknown-linux-gnu/lib/libcore-e8edd0fd.rlib" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util" "-l" "compiler-rt"
note: clang: warning: argument unused during compilation: '-pie'
ld: unknown option: --as-needed
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
error: Could not compile `hello`.

To learn more, run the command again with --verbose.
