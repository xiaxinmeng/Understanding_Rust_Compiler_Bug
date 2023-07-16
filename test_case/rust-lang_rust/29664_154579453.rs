
     Running `./custom-rustc.sh /Users/travis/.cargo/registry/src/github.com-0a35038f75765ae4/kernel32-sys-0.1.4/build.rs --crate-name build_script_build --crate-type bin -g --out-dir /Users/travis/build/danburkert/memmap-rs/target/debug/build/kernel32-sys-62844336d3806e02 --emit=dep-info,link -L dependency=/Users/travis/build/danburkert/memmap-rs/target/debug/deps -L dependency=/Users/travis/build/danburkert/memmap-rs/target/debug/deps --extern build=/Users/travis/build/danburkert/memmap-rs/target/debug/deps/libbuild-304afb6bdff23d72.rlib --cap-lints allow`

error: linking with `cc` failed: exit code: 1

error: linking with `cc` failed: exit code: 1

note: "cc" "-m64" "-L" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/danburkert/memmap-rs/target/debug/build/advapi32-sys-cfef7a1f30f1e5f6/build_script_build.0.o" "-o" "/Users/travis/build/danburkert/memmap-rs/target/debug/build/advapi32-sys-cfef7a1f30f1e5f6/build_script_build" "-Wl,-dead_strip" "-nodefaultlibs" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps/libbuild-304afb6bdff23d72.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/libstd-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/libcollections-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/librustc_unicode-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/librand-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/liballoc-35017696.rlib" "/Users/travis/rust/lib/note: "cc" "-m64" "-L" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/danburkert/memmap-rs/target/debug/build/kernel32-sys-62844336d3806e02/build_script_build.0.o" "-o" "/Users/travis/build/danburkert/memmap-rs/target/debug/build/kernel32-sys-62844336d3806e02/build_script_build" "-Wl,-dead_strip" "-nodefaultlibs" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps/libbuild-304afb6bdff23d72.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/libstd-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/libcollections-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/librustc_unicode-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/librand-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/liballoc-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lirb/liballoc_jemalloc-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/liblibc-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/libcore-35017696.rlib" "-L" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps" "-L" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps" "-L" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/danburkert/memmap-rs/.rust/lib/x86_64-apple-darwin" "-L" "/Users/travis/build/danburkert/memmap-rs/lib/x86_64-apple-darwin" "-l" "System" "-l" "pthread" "-l" "c" "-l" "m" "-mios-simulator-version-min=7.0" "-l" "compiler-rt"

note: ld: warning: directory not found for option '-L/Users/travis/build/danburkert/memmap-rs/.rust/lib/x86_64-apple-darwin'

ld: warning: dustlib/x86_64-apple-darwin/lib/liballoc_jemalloc-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib/liblibc-35017696.rlib" "/Users/travis/rust/lib/rustlib/x86_64-apirectory not found for option '-L/Users/travis/build/danburkert/memmap-rs/lib/x86_64-apple-darwin'

ld: building for iOS Simulator, but linking against dylib built for MacOSX file '/usr/lib/libSystem.dylib' for architecture x86_64

clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error

ple-darwin/lib/libcore-35017696.rlib" "-L" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps" "-L" "/Users/travis/build/danburkert/memmap-rs/target/debug/deps" "-L" "/Users/travis/rust/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/danburkert/memmap-rs/.rust/lib/x86_64-apple-darwin" "-L" "/Users/travis/build/danburkert/memmap-rs/lib/x86_64-apple-darwin" "-l" "System" "-l" "pthread" "-l" "c" "-l" "m" "-mBuild failed, waiting for other jobs to finish...

ios-simulator-version-min=7.0" "-l" "compiler-rt"

note: ld: warning: directory not found for option '-L/Users/travis/build/danburkert/memmap-rs/.rust/lib/x86_64-apple-darwin'

ld: warning: directory not found for option '-L/Users/travis/build/danburkert/memmap-rs/lib/x86_64-apple-darwin'

ld: building for iOS Simulator, but linking against dylib built for MacOSX file '/usr/lib/libSystem.dylib' for architecture x86_64

clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error

Could not compile `kernel32-sys`.

Caused by:

  Process didn't exit successfully: `./custom-rustc.sh /Users/travis/.cargo/registry/src/github.com-0a35038f75765ae4/kernel32-sys-0.1.4/build.rs --crate-name build_script_build --crate-type bin -g --out-dir /Users/travis/build/danburkert/memmap-rs/target/debug/build/kernel32-sys-62844336d3806e02 --emit=dep-info,link -L dependency=/Users/travis/build/danburkert/memmap-rs/target/debug/deps -L dependency=/Users/travis/build/danburkert/memmap-rs/target/debug/deps --extern build=/Users/travis/build/danburkert/memmap-rs/target/debug/deps/libbuild-304afb6bdff23d72.rlib --cap-lints allow` (exit code: 101)
