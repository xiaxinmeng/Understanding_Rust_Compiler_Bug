
manfred@xyz:_/src/rust$ rm ./build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/* 
manfred@xyz:~/src/rust$ ./x.py build
Updating submodules
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
   Compiling serde v1.0.15
   Compiling fnv v1.0.5
   Compiling log v0.3.8
   Compiling lazy_static v0.2.8
   Compiling string_cache_shared v0.3.0
   Compiling smallvec v0.3.3
   Compiling procedural-masquerade v0.1.2
   Compiling bitflags v0.8.2
   Compiling matches v0.1.6
   Compiling precomputed-hash v0.1.0
   Compiling mac v0.1.1
   Compiling bitflags v0.7.0
   Compiling void v1.0.2
   Compiling siphasher v0.2.2
   Compiling phf_generator v0.7.21
   Compiling pulldown-cmark v0.0.14
   Compiling unreachable v0.1.1
   Compiling utf-8 v0.7.1
error[E0460]: found possibly newer version of crate 'std' which 'phf_shared' depends on
 --> /home/manfred/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_generator-0.7.21/src/lib.rs:2:1
  |
2 | extern crate phf_shared;
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: perhaps that crate needs to be recompiled?
  = note: crate 'std' path #1: /home/manfred/src/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5e8ebc384e5dfd82.rlib
  = note: crate 'std' path #2: /home/manfred/src/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5e8ebc384e5dfd82.so
  = note: crate 'phf_shared' path #1: /home/manfred/src/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libphf_shared-52a660f4c9fcee8a.rlib

error: aborting due to previous error

error: Could not compile 'phf_generator'.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/home/manfred/src/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "24" "--release" "--manifest-path" "/home/manfred/src/rust/src/tools/rustdoc/Cargo.toml"
expected success, got: exit code: 101 

failed to run: /home/manfred/src/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:00:15
manfred@xyz:~/src/rust$ cd ..
