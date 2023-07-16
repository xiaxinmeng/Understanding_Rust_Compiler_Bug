plain
  SCRIPT: python x.py --stage 2 test src/tools/cargotest src/tools/cargo
  VCVARS_BAT: vcvars64.bat
  WIX: /d/a/rust/rust/wix
##[endgroup]
Junction created for build <<===>> c:\MORE_SPACE
src/ci/scripts/disable-git-crlf-conversion.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-cargo
---
[RUSTC-TIMING] rustdoc_tool_binary test:false 1.052
    Finished release [optimized] target(s) in 2m 32s
[TIMING] Rustdoc { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } } -- 152.568
testing https://github.com/diesel-rs/diesel
Initialized empty Git repository in C:/MORE_SPACE/ct/diesel/.git/
From https://github.com/diesel-rs/diesel
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object '91493fe47175076f330ce5fc518f0196c0476f56'.
From https://github.com/diesel-rs/diesel
---

test result: ok. 124 passed; 0 failed; 16 ignored; 0 measured; 0 filtered out; finished in 20.21s

testing https://github.com/servo/servo
Initialized empty Git repository in C:/MORE_SPACE/ct/servo/.git/
From https://github.com/servo/servo
 * branch                master     -> FETCH_HEAD
fatal: Could not parse object 'caac107ae8145ef2fd20365e2b8fadaf09c2eb3b'.
From https://github.com/servo/servo
---

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

testing https://github.com/BurntSushi/xsv
Initialized empty Git repository in C:/MORE_SPACE/ct/xsv/.git/
From https://github.com/BurntSushi/xsv
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object '3de6c04269a7d315f7e9864b9013451cd9580a08'.
From https://github.com/BurntSushi/xsv
---

test result: ok. 426 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.90s

testing https://github.com/XAMPPRocky/tokei
Initialized empty Git repository in C:/MORE_SPACE/ct/tokei/.git/
From https://github.com/XAMPPRocky/tokei
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object 'fdf3f8cb279a7aeac0696c87e5d8b0cd946e4f9e'.
From https://github.com/XAMPPRocky/tokei
---

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.42s

testing https://github.com/BurntSushi/ripgrep
Initialized empty Git repository in C:/MORE_SPACE/ct/ripgrep/.git/
From https://github.com/BurntSushi/ripgrep
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object '3de31f752729525d85a3d1575ac1978733b3f7e7'.
From https://github.com/BurntSushi/ripgrep
---

test result: ok. 229 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.17s

testing https://github.com/iron/iron
Initialized empty Git repository in C:/MORE_SPACE/ct/iron/.git/
From https://github.com/iron/iron
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object 'cf056ea5e8052c1feea6141e40ab0306715a2c33'.
From https://github.com/iron/iron
---
test workspaces::ws_warn_unused ... ok

failures:

---- lto::complicated stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v --release`
thread 'lto::complicated' panicked at '
test failed running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v --release`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
    Updating `dummy-registry` index
---
The targets should have unique names.
Consider changing their names to be unique or compiling them separately.
This may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>.
   Compiling dep-shared v0.0.1
   Compiling dep-build2 v0.0.1
   Compiling dep-proc-macro2 v0.0.1
   Compiling dep-normal2 v0.0.1
     Running `rustc --crate-name dep_shared D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-shared-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C metadata=0cafa35772366ade -C extra-filename=-0cafa35772366ade --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --cap-lints allow`
     Running `rustc --crate-name dep_build2 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-build2-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=6c14cb55da5ebfaa -C extra-filename=-6c14cb55da5ebfaa --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --cap-lints allow`
     Running `rustc --crate-name dep_proc_macro2 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-proc-macro2-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=12a250fd2712894f -C extra-filename=-12a250fd2712894f --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --cap-lints allow`
     Running `rustc --crate-name dep_normal2 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-normal2-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=9445c01b08693af1 -C extra-filename=-9445c01b08693af1 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --cap-lints allow`
   Compiling dep-build v0.0.1
   Compiling dep-normal v0.0.1
     Running `rustc --crate-name dep_normal D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-normal-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=b131715d4827f864 -C extra-filename=-b131715d4827f864 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_normal2=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_normal2-9445c01b08693af1.rmeta --extern dep_shared=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_shared-0cafa35772366ade.rmeta --cap-lints allow`
     Running `rustc --crate-name dep_build D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-build-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=42e3c2ffc4679603 -C extra-filename=-42e3c2ffc4679603 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_build2=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_build2-6c14cb55da5ebfaa.rmeta --extern dep_shared=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_shared-0cafa35772366ade.rmeta --cap-lints allow`
   Compiling dep-proc-macro v0.0.1
     Running `rustc --crate-name dep_proc_macro D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\home\.cargo\registry\src\-45f4c251e71fd8a7\dep-proc-macro-0.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C metadata=07417f7370b13e1d -C extra-filename=-07417f7370b13e1d --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_proc_macro2=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_proc_macro2-12a250fd2712894f.rlib --extern dep_shared=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_shared-0cafa35772366ade.rlib --extern proc_macro --cap-lints allow`
   Compiling test v0.0.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo)
     Running `rustc --crate-name build_script_build build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no -C metadata=4254b5ee8af3e8b8 -C extra-filename=-4254b5ee8af3e8b8 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\build\test-4254b5ee8af3e8b8 -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_build=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_build-42e3c2ffc4679603.rlib`
     Running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\build\test-4254b5ee8af3e8b8\build-script-build`
     Running `rustc --crate-name test src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C metadata=a65517f84bef4706 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_normal=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_normal-b131715d4827f864.rlib --extern dep_proc_macro=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\dep_proc_macro-07417f7370b13e1d.dll`
     Running `rustc --crate-name test src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type cdylib --crate-type staticlib --emit=dep-info,link -C opt-level=3 -C lto -C metadata=966d9f13a276c54c --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_normal=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_normal-b131715d4827f864.rlib --extern dep_proc_macro=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\dep_proc_macro-07417f7370b13e1d.dll`
error: linking with `link.exe` failed: exit code: 1201
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\tmp\\cit\\t1300\\foo\\target\\release\\deps\\test.test.3ad58348-cgu.1.rcgu.o" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\tmp\\cit\\t1300\\foo\\target\\release\\deps" "/LIBPATH:C:\\MORE_SPACE\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\MORE_SPACE\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-c606d30d5817c852.rlib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "advapi32.lib" "userenv.lib" "kernel32.lib" "msvcrt.lib" "/NXCOMPAT" "/LIBPATH:C:\\MORE_SPACE\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\tmp\\cit\\t1300\\foo\\target\\release\\deps\\test.exe" "/OPT:REF,ICF" "/DEBUG"
  = note: LINK : fatal error LNK1201: error writing to program database 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\test.pdb'; check for insufficient disk space, invalid path, or insufficient privilege

error: could not compile `test` due to previous error

Caused by:
Caused by:
  process didn't exit successfully: `rustc --crate-name test src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C metadata=a65517f84bef4706 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps --extern dep_normal=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\libdep_normal-b131715d4827f864.rlib --extern dep_proc_macro=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\dep_proc_macro-07417f7370b13e1d.dll` (exit code: 1)
error: build failed
', src\tools\cargo\tests\testsuite\lto.rs:217:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---

error: test failed, to rerun pass '--test testsuite'


command did not execute successfully: "\\\\?\\C:\\MORE_SPACE\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\rust\\rust\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"


Build completed unsuccessfully in 0:53:51
