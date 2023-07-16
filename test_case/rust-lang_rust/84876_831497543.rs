plain
[RUSTC-TIMING] chalk_ir test:false 0.710
error: could not compile `chalk-ir`

Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap/debug/rustc --crate-name chalk_ir --edition=2018 C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\chalk-ir-0.55.0\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=d12f81ffd2bd1d14 -C extra-filename=-d12f81ffd2bd1d14 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps --extern bitflags=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\libbitflags-27bafe7035184673.rmeta --extern chalk_derive=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps\chalk_derive-82a7839463292e1c.dll --extern lazy_static=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\deps\liblazy_static-533fc5e5c4427266.rmeta --cap-lints allow -Zmacro-backtrace -Ctarget-feature=+crt-static -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic -Z binary-dep-depinfo` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
[RUSTC-TIMING] rustc_macros test:false 2.375
[RUSTC-TIMING] tracing test:false 0.644
[RUSTC-TIMING] regex test:false 11.614
[RUSTC-TIMING] serde_derive test:false 5.578
[RUSTC-TIMING] serde_derive test:false 5.578
error: build failed
command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "D:\\a\\rust\\rust\\compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:14:38
