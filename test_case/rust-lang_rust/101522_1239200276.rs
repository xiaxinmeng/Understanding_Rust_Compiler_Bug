plain
[RUSTC-TIMING] gimli test:false 3.884
    Checking addr2line v0.16.0
[RUSTC-TIMING] addr2line test:false 0.411
[RUSTC-TIMING] object test:false 5.059
thread 'rustc' panicked at 'range start index 1 out of range for slice of length 0', compiler\rustc_middle\src\mir\mod.rs:1473:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d3f6742fe 2022-09-07) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type dylib --crate-type rlib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C debug-assertions=off -C overflow-checks=on -Z force-unstable-if-unmarked -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `sys::windows::fs::<impl at D:\a\rust\rust\library\std\src\sys\windows\fs.rs:278:1: 278:10>::file_attr`
#1 [optimized_mir] optimizing MIR for `sys::windows::fs::<impl at D:\a\rust\rust\library\std\src\sys\windows\fs.rs:278:1: 278:10>::file_attr`
[RUSTC-TIMING] std test:false 5.882
error: could not compile `std`

Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap\debug\rustc --crate-name std --edition=2021 D:\a\rust\rust\library\std\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type dylib --crate-type rlib --emit=dep-info,metadata -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on --cfg "feature=\"addr2line\"" --cfg "feature=\"backtrace\"" --cfg "feature=\"gimli-symbolize\"" --cfg "feature=\"miniz_oxide\"" --cfg "feature=\"object\"" --cfg "feature=\"panic_unwind\"" --cfg "feature=\"std_detect_dlsym_getauxval\"" --cfg "feature=\"std_detect_file_io\"" -C metadata=442352e442e9932f -C extra-filename=-442352e442e9932f --out-dir C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps -L dependency=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\release\deps --extern addr2line=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libaddr2line-1917949538ea489e.rmeta --extern alloc=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\liballoc-e154135ad4d54be0.rmeta --extern cfg_if=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libcfg_if-845d537aba9250e6.rmeta --extern compiler_builtins=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libcompiler_builtins-703be3adaa38573f.rmeta --extern core=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libcore-cd8743a1ce45de64.rmeta --extern hashbrown=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libhashbrown-926e18b9e7d03614.rmeta --extern libc=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\liblibc-66152e66d7d1eeca.rmeta --extern miniz_oxide=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libminiz_oxide-acaaaf9fe8e28d0d.rmeta --extern object=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libobject-a0118d5eb9b76878.rmeta --extern panic_abort=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libpanic_abort-ac2cdd1c4aa6ff40.rmeta --extern panic_unwind=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libpanic_unwind-d5b7dc5342c74f26.rmeta --extern rustc_demangle=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\librustc_demangle-33d5e210d61946f5.rmeta --extern std_detect=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libstd_detect-320e36899327c5df.rmeta --extern unwind=C:\Users\RUNNER~1\AppData\Local\Temp\xargo.xaCdWv2xZX5g\target\x86_64-pc-windows-msvc\release\deps\libunwind-4228dfff85cd8ecf.rmeta -Cdebug-assertions=off -Coverflow-checks=on -Z force-unstable-if-unmarked --sysroot C:\Users\runneradmin\AppData\Local\rust-lang\miri\cache\HOST --cfg backtrace_in_libstd` (exit code: 0x80000003)
error: `"\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "check" "--release" "--manifest-path" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\xargo.xaCdWv2xZX5g\\Cargo.toml" "--target" "x86_64-pc-windows-msvc" "-p" "std"` failed with exit code: Some(101)
error: process didn't exit successfully: `build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo-miri.exe miri setup` (exit code: 1)
[TIMING] test::Miri { stage: 2, host: x86_64-pc-windows-msvc } -- 57.553

1 command(s) did not execute successfully:
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:01
