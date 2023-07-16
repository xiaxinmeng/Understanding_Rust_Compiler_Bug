
[01:07:15] error[E0463]: can't find crate for `rustc_const_math`
[01:07:15]   --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\clippy_lints-0.0.195\src\lib.rs:41:1
[01:07:15]    |
[01:07:15] 41 | extern crate rustc_const_math;
[01:07:15]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:07:15] 
[01:07:15] error: aborting due to previous error
[01:07:15] 
[01:07:15] For more information about this error, try `rustc --explain E0463`.
[01:07:15] [RUSTC-TIMING] clippy_lints test:false 1.479
[01:07:15] error: Could not compile `clippy_lints`.
[01:07:15] 
[01:07:15] Caused by:
[01:07:15]   process didn't exit successfully: `C:\projects\rust\build\bootstrap/debug/rustc --crate-name clippy_lints C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\clippy_lints-0.0.195\src\lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=f034645c1cef2be5 -C extra-filename=-f034645c1cef2be5 --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\release\deps --extern pulldown_cmark=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libpulldown_cmark-a0b0aaf454852239.rlib --extern quine_mc_cluskey=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libquine_mc_cluskey-92221a71029eb882.rlib --extern url=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\liburl-d6e8656eb2fa755b.rlib --extern regex_syntax=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libregex_syntax-e7ec8710b24436c4.rlib --extern if_chain=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libif_chain-2d9ae78132625b27.rlib --extern unicode_normalization=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libunicode_normalization-6989f445ddc87a84.rlib --extern semver=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libsemver-325d1ba6e49fa23c.rlib --extern itertools=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libitertools-e06725263431b900.rlib --extern serde_derive=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\release\deps\serde_derive-1e2188502da7e9f0.dll --extern lazy_static=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\liblazy_static-56230eaac815c346.rlib --extern serde=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libserde-fdab64f98513f90c.rlib --extern toml=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libtoml-3311258f5e47c7f5.rlib --extern matches=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\libmatches-d3dda493b0e1b7b5.rlib --cap-lints allow` (exit code: 101)
[01:07:15] warning: build failed, waiting for other jobs to finish...
[01:07:19] [RUSTC-TIMING] rayon test:false 4.934
[01:07:33] [RUSTC-TIMING] rustc_target test:false 19.163
[01:08:04] error: build failed
[01:08:04] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls\\Cargo.toml" "--features" "clippy" "--message-format" "json"
[01:08:04] expected success, got: exit code: 101
[01:08:04] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", tool: "rls", path: "src/tools/rls", mode: Librustc, is_ext_tool: true, extra_features: ["clippy"] } -- 299.140
[01:08:04] failed to test rls: could not build
