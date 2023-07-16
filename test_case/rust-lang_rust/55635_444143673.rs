plain
[00:02:24]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:02:24]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:02:25]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:02:25]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     |
[00:02:34]     |
[00:02:34] 73  |                       $Ty(unsafe { NonZero(n) })
[00:02:34]     |                           ^^^^^^ unnecessary `unsafe` block
[00:02:34] ...
[00:02:34] 110 | / nonzero_integers! {
[00:02:34] 111 | |     NonZeroU8(u8);
[00:02:34] 112 | |     NonZeroU16(u16);
[00:02:34] 113 | |     NonZeroU32(u32);
[00:02:34] 116 | |     NonZeroUsize(usize);
[00:02:34] 117 | | }
[00:02:34]     | |_- in this macro invocation
[00:02:34]     |
[00:02:34]     |
[00:02:34]     = note: #[warn(unused_unsafe)] on by default
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     |
[00:02:34]     |
[00:02:34] 73  |                       $Ty(unsafe { NonZero(n) })
[00:02:34]     |                           ^^^^^^ unnecessary `unsafe` block
[00:02:34] ...
[00:02:34] 110 | / nonzero_integers! {
[00:02:34] 111 | |     NonZeroU8(u8);
[00:02:34] 112 | |     NonZeroU16(u16);
[00:02:34] 113 | |     NonZeroU32(u32);
[00:02:34] 116 | |     NonZeroUsize(usize);
[00:02:34] 117 | | }
[00:02:34]     | |_- in this macro invocation
[00:02:34] 
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     |
[00:02:34]     |
[00:02:34] 81  |                           Some($Ty(unsafe { NonZero(n) }))
[00:02:34]     |                                    ^^^^^^ unnecessary `unsafe` block
[00:02:34] ...
[00:02:34] 110 | / nonzero_integers! {
[00:02:34] 111 | |     NonZeroU8(u8);
[00:02:34] 112 | |     NonZeroU16(u16);
[00:02:34] 113 | |     NonZeroU32(u32);
[00:02:34] 116 | |     NonZeroUsize(usize);
[00:02:34] 117 | | }
[00:02:34]     | |_- in this macro invocation
[00:02:34] 
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:2762:36
[00:02:34]      |
[00:02:34] 2762 |             Some(Unique { pointer: unsafe { NonZero(ptr as _) }, _marker: PhantomData })
[00:02:34]      |                                    ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:2818:27
[00:02:34]      |
[00:02:34] 2818 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:02:34]      |                           ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:2825:27
[00:02:34]      |
[00:02:34] 2825 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:02:34]      |                           ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:2898:28
[00:02:34]      |
[00:02:34] 2898 |         NonNull { pointer: unsafe { NonZero(ptr as _) } }
[00:02:34]      |                            ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:3028:28
[00:02:34]      |
[00:02:34] 3028 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:02:34]      |                            ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]     --> src/libcore/ptr.rs:3036:28
[00:02:34]      |
[00:02:34] 3036 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:02:34]      |                            ^^^^^^ unnecessary `unsafe` block
[00:02:34] 
[00:02:34] warning: unnecessary `unsafe` block
[00:02:34]    |
[00:02:34]    |
[00:02:34] 27 |         unsafe { NonZero(self.0) }
[00:02:34]    |         ^^^^^^ unnecessary `unsafe` block
[00:02:39] [RUSTC-TIMING] core test:false 24.101
[00:02:41] [RUSTC-TIMING] compiler_builtins test:false 1.844
[00:02:41]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:02:41]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
---
[01:09:42] [RUSTC-TIMING] arena test:false 0.926
[01:09:42]    Compiling rustc-ap-syntax_pos v297.0.0
[01:09:44] [RUSTC-TIMING] git2_curl test:false 2.904
[01:09:44]    Compiling cargo v0.33.0 (/checkout/src/tools/cargo)
[01:09:45] error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block
[01:09:45]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_target-297.0.0/abi/mod.rs:830:1
[01:09:45]     |
[01:09:45] 830 | / newtype_index! {
[01:09:45] 831 | |     pub struct VariantIdx { .. }
[01:09:45] 832 | | }
[01:09:45]     | |_^ initializing type with `rustc_layout_scalar_valid_range` attr
[01:09:45]     |
[01:09:45]     = note: initializing a layout restricted type's field with a value outside the valid range is undefined behavior
[01:09:45] 
[01:09:45] 
[01:09:45] error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block
[01:09:45]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_target-297.0.0/abi/mod.rs:830:1
[01:09:45]     |
[01:09:45] 830 | / newtype_index! {
[01:09:45] 831 | |     pub struct VariantIdx { .. }
[01:09:45] 832 | | }
[01:09:45]     | |_^ initializing type with `rustc_layout_scalar_valid_range` attr
[01:09:45]     |
[01:09:45]     = note: initializing a layout restricted type's field with a value outside the valid range is undefined behavior
[01:09:45] 
[01:09:45] error: aborting due to 2 previous errors
[01:09:45] 
[01:09:45] For more information about this error, try `rustc --explain E0133`.
---
[01:12:59]    Compiling rustc-ap-arena v297.0.0
[01:12:59]    Compiling rustc-ap-rustc_target v297.0.0
[01:12:59] [RUSTC-TIMING] arena test:false 0.528
[01:12:59]    Compiling rustc-ap-syntax_pos v297.0.0
[01:13:02] error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block
[01:13:02]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_target-297.0.0/abi/mod.rs:830:1
[01:13:02]     |
[01:13:02] 830 | / newtype_index! {
[01:13:02] 831 | |     pub struct VariantIdx { .. }
[01:13:02] 832 | | }
[01:13:02]     | |_^ initializing type with `rustc_layout_scalar_valid_range` attr
[01:13:02]     |
[01:13:02]     = note: initializing a layout restricted type's field with a value outside the valid range is undefined behavior
[01:13:02] 
[01:13:02] 
[01:13:02] error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block
[01:13:02]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_target-297.0.0/abi/mod.rs:830:1
[01:13:02]     |
[01:13:02] 830 | / newtype_index! {
[01:13:02] 831 | |     pub struct VariantIdx { .. }
[01:13:02] 832 | | }
[01:13:02]     | |_^ initializing type with `rustc_layout_scalar_valid_range` attr
[01:13:02]     |
[01:13:02]     = note: initializing a layout restricted type's field with a value outside the valid range is undefined behavior
[01:13:02] 
[01:13:02] error: aborting due to 2 previous errors
[01:13:02] 
[01:13:02] For more information about this error, try `rustc --explain E0133`.
---
[01:16:14] failures:
[01:16:14] 
[01:16:14] ---- [compile-fail] compile-fail/validity/nonzero.rs stdout ----
[01:16:14] 
[01:16:14] error: tests/compile-fail/validity/nonzero.rs:10: unexpected error: '10:19: 10:29: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block [E0133]'
[01:16:14] 
[01:16:14] error: tests/compile-fail/validity/nonzero.rs:10: expected error not found: encountered 0, but expected something greater or equal to 1
[01:16:14] error: 1 unexpected errors found, 1 expected errors not found
[01:16:14] status: exit code: 1
[01:16:14] status: exit code: 1
[01:16:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/nonzero.rs" "-L" "/tmp/compiletestvu0kh1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestvu0kh1/validity/nonzero.stage-id" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestvu0kh1/validity/nonzero.stage-id.aux" "-A" "unused"
[01:16:14] unexpected errors (from JSON output): [
[01:16:14]     Error {
[01:16:14]         line_num: 10,
[01:16:14]         kind: Some(
[01:16:14]         kind: Some(
[01:16:14]             Error
[01:16:14]         ),
[01:16:14]         msg: "10:19: 10:29: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block [E0133]"
[01:16:14] ]
[01:16:14] 
[01:16:14] not found errors (from test file): [
[01:16:14]     Error {
[01:16:14]     Error {
[01:16:14]         line_num: 10,
[01:16:14]         kind: Some(
[01:16:14]             Error
[01:16:14]         ),
[01:16:14]         msg: "encountered 0, but expected something greater or equal to 1"
[01:16:14] ]
[01:16:14] 
[01:16:14] thread '[compile-fail] compile-fail/validity/nonzero.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.17/src/runtest.rs:1098:13
[01:16:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
travis_time:end:094631be:start=1543937712944865554,finish=1543937712951008419,duration=6142865
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00efd01b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00a40e95
travis_time:start:00a40e95
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:075b2172
$ dmesg | grep -i kill
