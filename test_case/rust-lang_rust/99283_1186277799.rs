plain
[RUSTC-TIMING] textwrap test:false 2.802
[RUSTC-TIMING] rustc_hash test:false 0.088
   Compiling rustc-demangle v0.1.21
   Compiling indenter v0.3.3
   Compiling owo-colors v3.4.0
[RUSTC-TIMING] shell_escape test:false 0.957
[RUSTC-TIMING] diff test:false 3.188
   Compiling ansi_term v0.12.1
[RUSTC-TIMING] openssl_src test:false 0.301
---
[RUSTC-TIMING] eyre test:false 1.136
   Compiling tracing-subscriber v0.3.3
[RUSTC-TIMING] sharded_slab test:false 2.068
[RUSTC-TIMING] rustc_demangle test:false 2.613
[RUSTC-TIMING] owo_colors test:false 3.198
   Compiling tracing-error v0.2.0
   Compiling color-spantrace v0.2.0
[RUSTC-TIMING] tracing_error test:false 0.310
[RUSTC-TIMING] color_spantrace test:false 0.764
[RUSTC-TIMING] color_spantrace test:false 0.764
[RUSTC-TIMING] tracing_subscriber test:false 3.840
   Compiling addr2line v0.17.0
[RUSTC-TIMING] addr2line test:false 0.711
[RUSTC-TIMING] gimli test:false 7.285
   Compiling color-eyre v0.6.2
[RUSTC-TIMING] object test:false 8.803
[RUSTC-TIMING] backtrace test:false 3.302
[RUSTC-TIMING] backtrace test:false 3.302
[RUSTC-TIMING] color_eyre test:false 2.789
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
[RUSTC-TIMING] compiletest test:false 1.453
[RUSTC-TIMING] miri test:true 8.012
    Finished release [optimized] target(s) in 22.49s
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/stacked_borrows/issue-miri-1050-1.rs" "--error-format=json"

actual output differed from expected tests/fail/stacked_borrows/issue-miri-1050-1.stderr
Diff < left / right > :
>warning: unused return value of `std::boxed::Box::<T>::from_raw` that must be used
>   |
>LL |         Box::from_raw(ptr as *mut u32);
>   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>   |
>   |
>   = note: `#[warn(unused_must_use)]` on by default
>   = note: call `drop(from_raw(ptr))` if you intend to drop the `Box`
>
 error: Undefined Behavior: out-of-bounds pointer use: ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
    |
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::boxed::Box::<u32>::from_raw_in` at RUSTLIB/alloc/src/boxed.rs:LL:CC
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/stacked_borrows/issue-miri-1050-2.rs" "--error-format=json"

actual output differed from expected tests/fail/stacked_borrows/issue-miri-1050-2.stderr
Diff < left / right > :
>warning: unused return value of `std::boxed::Box::<T>::from_raw` that must be used
>   |
>LL |         Box::from_raw(ptr.as_ptr());
>   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>   |
>   |
>   = note: `#[warn(unused_must_use)]` on by default
>   = note: call `drop(from_raw(ptr))` if you intend to drop the `Box`
>
 error: Undefined Behavior: out-of-bounds pointer use: 0x4[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: 0x4[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::boxed::Box::<i32>::from_raw_in` at RUSTLIB/alloc/src/boxed.rs:LL:CC
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
