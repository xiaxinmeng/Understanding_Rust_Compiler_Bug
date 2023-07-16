plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:0e31b9a0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:42]    Compiling scopeguard v0.3.3
[00:06:42]    Compiling libc v0.2.40
[00:06:42]    Compiling memoffset v0.2.1
[00:06:42]    Compiling lazy_static v1.0.0
[00:06:42]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:06:42]    Compiling stable_deref_trait v1.0.0
[00:06:42]    Compiling either v1.5.0
[00:06:43]    Compiling bitflags v1.0.1
[00:06:43]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:07:18]    Compiling parking_lot v0.5.5
[00:07:18]    Compiling crossbeam-deque v0.2.0
[00:07:18]    Compiling rls-data v0.15.0
[00:07:31]    Compiling flate2 v1.0.1
[00:07:31]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:07:35]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:07:37]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:07:38]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:07:42]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:34:24]    Compiling memoffset v0.2.1
[00:34:24]    Compiling libc v0.2.40
[00:34:24]    Compiling scopeguard v0.3.3
[00:34:24]    Compiling lazy_static v1.0.0
[00:34:24]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:34:24]    Compiling stable_deref_trait v1.0.0
[00:34:24]    Compiling bitflags v1.0.1
[00:34:25]    Compiling either v1.5.0
[00:34:25]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:34:59]    Compiling crossbeam-deque v0.2.0
[00:35:00]    Compiling rls-data v0.15.0
[00:35:00]    Compiling parking_lot v0.5.5
[00:35:02]    Compiling flate2 v1.0.1
[00:35:09]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:35:15]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:35:17]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:35:18]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:35:21]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:58:59]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:58:59]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:58:59]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:59:00]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:59:00] error: Could not compile `panic_abort`.
[00:59:03] error: build failed
[00:59:03] 
[00:59:03] 
[00:59:03] 
[00:59:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:59:03] 
[00:59:03] 
[00:59:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:59:03] Build completed unsuccessfully in 0:53:35
