
142  | |         #[doc = concat!("[`ilog2`]: ", stringify!($SelfT), "::ilog2")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`ilog2`]: u128::ilog2
                        ^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `ilog2`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: public documentation for `unchecked_add` links to private item `u128::checked_add`
    --> library/core/src/num/uint_macros.rs:465:9
     |
465  | /         /// Unchecked integer addition. Computes `self + rhs`, assuming overflow
466  | |         /// cannot occur.
467  | |         ///
468  | |         /// # Safety
473  | |         ///
473  | |         ///
474  | |         #[doc = concat!("[`checked_add`]: ", stringify!($SelfT), "::checked_add")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`checked_add`]: u128::checked_add
     = note: this link will resolve properly if you pass `--document-private-items`
     = note: this link will resolve properly if you pass `--document-private-items`
     = note: `-D rustdoc::private-intra-doc-links` implied by `-D warnings`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: public documentation for `unchecked_sub` links to private item `u128::checked_sub`
    --> library/core/src/num/uint_macros.rs:534:9
     |
534  | /         /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow
535  | |         /// cannot occur.
536  | |         ///
537  | |         /// # Safety
542  | |         ///
542  | |         ///
543  | |         #[doc = concat!("[`checked_sub`]: ", stringify!($SelfT), "::checked_sub")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`checked_sub`]: u128::checked_sub
     = note: this link will resolve properly if you pass `--document-private-items`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)


error: public documentation for `unchecked_mul` links to private item `u128::checked_mul`
    --> library/core/src/num/uint_macros.rs:581:9
     |
581  | /         /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
582  | |         /// cannot occur.
583  | |         ///
584  | |         /// # Safety
589  | |         ///
589  | |         ///
590  | |         #[doc = concat!("[`checked_mul`]: ", stringify!($SelfT), "::checked_mul")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`checked_mul`]: u128::checked_mul
     = note: this link will resolve properly if you pass `--document-private-items`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)


error: unresolved link to `u128::checked_shl`
    --> library/core/src/num/uint_macros.rs:918:9
     |
918  | /         /// Unchecked shift left. Computes `self << rhs`, assuming that
919  | |         /// `rhs` is less than the number of bits in `self`.
920  | |         ///
921  | |         /// # Safety
926  | |         ///
926  | |         ///
927  | |         #[doc = concat!("[`checked_shl`]: ", stringify!($SelfT), "::checked_shl")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`checked_shl`]: u128::checked_shl
                              ^^^^^^^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `checked_shl`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `u128::checked_shr`
    --> library/core/src/num/uint_macros.rs:966:9
     |
966  | /         /// Unchecked shift right. Computes `self >> rhs`, assuming that
967  | |         /// `rhs` is less than the number of bits in `self`.
968  | |         ///
969  | |         /// # Safety
974  | |         ///
974  | |         ///
975  | |         #[doc = concat!("[`checked_shr`]: ", stringify!($SelfT), "::checked_shr")]
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
1117 | /     uint_impl! {
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`checked_shr`]: u128::checked_shr
                              ^^^^^^^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `checked_shr`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `Self::rotate_left`
    --> library/core/src/num/uint_macros.rs:1382:9
     |
     |
1382 | /         /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
1383 | |         /// where `mask` removes any high-order bits of `rhs` that
1384 | |         /// would cause the shift to exceed the bitwidth of the type.
...    |
...    |
1399 | |         #[doc = concat!("assert_eq!(1", stringify!($SelfT), ".wrapping_shl(128), 1);")]
     | |_______________^
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             types all implement a [`rotate_left`](Self::rotate_left) function,
                                                   ^^^^^^^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `rotate_left`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `Self::rotate_right`
    --> library/core/src/num/uint_macros.rs:1415:9
     |
     |
1415 | /         /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
1416 | |         /// where `mask` removes any high-order bits of `rhs` that
1417 | |         /// would cause the shift to exceed the bitwidth of the type.
...    |
...    |
1432 | |         #[doc = concat!("assert_eq!(128", stringify!($SelfT), ".wrapping_shr(128), 128);")]
     | |_______________^
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             types all implement a [`rotate_right`](Self::rotate_right) function,
                                                    ^^^^^^^^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `rotate_right`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `Self::overflowing_add`
    --> library/core/src/num/uint_macros.rs:1510:9
     |
     |
1510 | /         /// Calculates `self` + `rhs` + `carry` and returns a tuple containing
1511 | |         /// the sum and the output carry.
1512 | |         ///
1513 | |         /// Performs "ternary addition" of two integer operands and a carry-in
...    |
1545 | |         /// assert_eq!((sum1, sum0), (9, 6));
     | |_______________^
     |
    ::: library/core/src/num/mod.rs:1117:5
     |
---
     | |_____- in this macro invocation
     |
     = note: the link appears in this line:
             
             [`overflowing_add`](Self::overflowing_add), and the output carry is
                                 ^^^^^^^^^^^^^^^^^^^^^
     = note: the builtin type `u128` has no associated item named `overflowing_add`
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.70.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=97357397889e81ef -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' '--check-cfg=values(target_env,"ohos")' -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=1 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (e5e773fca
  2023-03-30)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
stage-build INFO: Section `Stage 4 (final build)` ended: FAIL (145.65s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
------------------------------------------------
---
Total duration:                        1h 24m 6s
------------------------------------------------
root INFO: Free disk space: 475.97 GiB out of total 581.32 GiB (18.12% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 818, in execute_build_pipeline
    cmd(final_build_args)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['python3', '../x.py', 'dist', '--host', 'x86_64-unknown-linux-gnu', '--target', 'x86_64-unknown-linux-gnu', '--include-default-paths', 'build-manifest', 'bootstrap', '--llvm-profile-use', '/tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata', '--rust-profile-use', '/tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata', '--llvm-bolt-profile-use', '/tmp/tmp-multistage/opt-artifacts/bolt.profdata']' returned non-zero exit status 1.
