plain
    |
36  |   macro_rules! assert_eq {
    |   ---------------------- in this expansion of `$crate::assert_eq!` (#2)
...
53  |                   if !(*left_val == *right_val) {
    |                                     ^^^^^^^^^^ expected `(Size, AbiAndPrefAlign)`, found `(Size, Align)`
246 |   macro_rules! debug_assert_eq {
    |   ---------------------------- in this expansion of `debug_assert_eq!` (#1)
...
249 |               $crate::assert_eq!($($arg)*);
249 |               $crate::assert_eq!($($arg)*);
    |               ---------------------------- in this macro invocation (#2)
    |
   ::: compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:185:13
    |
185 | /             debug_assert_eq!(
186 | |                 (data_layout.pointer_size, data_layout.pointer_align),
187 | |                 cx.size_and_align_of(ptr_type),
188 | |                 "ptr_type={}, pointee_type={}",
189 | |                 ptr_type,
190 | |                 pointee_type,
    | |_____________- in this macro invocation (#1)
    |
    |
    = note: expected tuple `(rustc_target::abi::Size, AbiAndPrefAlign)`
               found tuple `(rustc_target::abi::Size, Align)`

error[E0599]: no method named `bits` found for struct `AbiAndPrefAlign` in the current scope
   --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:198:47
    |
198 |                     data_layout.pointer_align.bits() as u32,
    |                                               ^^^^ method not found in `AbiAndPrefAlign`
help: some of the expressions' fields have a method of the same name
    |
    |
198 |                     data_layout.pointer_align.abi.bits() as u32,
    |                                               ++++
198 |                     data_layout.pointer_align.pref.bits() as u32,

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_codegen_llvm test:false 1.947
---
Total duration:                           9m 16s
------------------------------------------------
root INFO: Free disk space: 510.00 GiB out of total 581.32 GiB (12.27% used)
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
