plain
   |
50 | / macro_rules! impl_read_unsigned_leb128 {
51 | |     ($fn_name:ident, $int_ty:ty) => {
52 | |         #[inline]
53 | |         pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
54 | |             #[inline]
55 | |             fn inner(slice: &[u8], position: &mut usize) -> Option<$int_ty> {
   | |                                                             --------------- expected `Option<u16>` because of return type
65 | |                     return byte as $int_ty;
   | |                            ^^^^ expected `Option<u16>`, found `u16`
...  |
84 | |     };
---
   = note: expected enum `Option<u16>`
              found type `u16`
help: try wrapping the expression in `Some`
   |
65 |                     return Some(byte) as $int_ty;

[RUSTC-TIMING] md5 test:false 0.097
   Compiling sha2 v0.10.6
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> compiler/rustc_serialize/src/leb128.rs:65:28
   |
50 | / macro_rules! impl_read_unsigned_leb128 {
51 | |     ($fn_name:ident, $int_ty:ty) => {
52 | |         #[inline]
53 | |         pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
54 | |             #[inline]
55 | |             fn inner(slice: &[u8], position: &mut usize) -> Option<$int_ty> {
   | |                                                             --------------- expected `Option<u32>` because of return type
65 | |                     return byte as $int_ty;
   | |                            ^^^^ expected `Option<u32>`, found `u32`
...  |
84 | |     };
---
   = note: expected enum `Option<u32>`
              found type `u32`
help: try wrapping the expression in `Some`
   |
65 |                     return Some(byte) as $int_ty;

error[E0308]: mismatched types
  --> compiler/rustc_serialize/src/leb128.rs:65:28
   |
   |
50 | / macro_rules! impl_read_unsigned_leb128 {
51 | |     ($fn_name:ident, $int_ty:ty) => {
52 | |         #[inline]
53 | |         pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
54 | |             #[inline]
55 | |             fn inner(slice: &[u8], position: &mut usize) -> Option<$int_ty> {
   | |                                                             --------------- expected `Option<u64>` because of return type
65 | |                     return byte as $int_ty;
   | |                            ^^^^ expected `Option<u64>`, found `u64`
...  |
84 | |     };
---
   = note: expected enum `Option<u64>`
              found type `u64`
help: try wrapping the expression in `Some`
   |
65 |                     return Some(byte) as $int_ty;

error[E0308]: mismatched types
  --> compiler/rustc_serialize/src/leb128.rs:65:28
   |
   |
50 | / macro_rules! impl_read_unsigned_leb128 {
51 | |     ($fn_name:ident, $int_ty:ty) => {
52 | |         #[inline]
53 | |         pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
54 | |             #[inline]
55 | |             fn inner(slice: &[u8], position: &mut usize) -> Option<$int_ty> {
   | |                                                             --------------- expected `Option<u128>` because of return type
65 | |                     return byte as $int_ty;
   | |                            ^^^^ expected `Option<u128>`, found `u128`
...  |
84 | |     };
---
   = note: expected enum `Option<u128>`
              found type `u128`
help: try wrapping the expression in `Some`
   |
65 |                     return Some(byte) as $int_ty;

error[E0308]: mismatched types
  --> compiler/rustc_serialize/src/leb128.rs:65:28
   |
   |
50 | / macro_rules! impl_read_unsigned_leb128 {
51 | |     ($fn_name:ident, $int_ty:ty) => {
52 | |         #[inline]
53 | |         pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
54 | |             #[inline]
55 | |             fn inner(slice: &[u8], position: &mut usize) -> Option<$int_ty> {
   | |                                                             --------------- expected `Option<usize>` because of return type
65 | |                     return byte as $int_ty;
   | |                            ^^^^ expected `Option<usize>`, found `usize`
...  |
84 | |     };
---
   = note: expected enum `Option<usize>`
              found type `usize`
help: try wrapping the expression in `Some`
   |
65 |                     return Some(byte) as $int_ty;

[RUSTC-TIMING] build_script_build test:false 0.239
   Compiling sha1 v0.10.5
For more information about this error, try `rustc --explain E0308`.
---
Total duration:                           7m 22s
------------------------------------------------
root INFO: Free disk space: 510.77 GiB out of total 581.32 GiB (12.13% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
