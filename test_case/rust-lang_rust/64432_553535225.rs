plain
2019-11-13T18:18:33.0094465Z [RUSTC-TIMING] lazycell test:false 0.136
2019-11-13T18:18:33.0331688Z    Compiling hex v0.4.0
2019-11-13T18:18:36.6904782Z [RUSTC-TIMING] hex test:false 3.659
2019-11-13T18:18:36.6976132Z    Compiling glob v0.3.0
2019-11-13T18:18:37.1936359Z memory allocation of 4294967304 bytes failed[RUSTC-TIMING] hex test:false 4.536
2019-11-13T18:18:37.1989695Z error: could not compile `hex`.
2019-11-13T18:18:37.1990887Z Caused by:
2019-11-13T18:18:37.1990887Z Caused by:
2019-11-13T18:18:37.1991486Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.3.2\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 -C metadata=d67f5eafaf62238d -C extra-filename=-d67f5eafaf62238d --out-dir D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps --target x86_64-pc-windows-gnu -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-13T18:18:38.6403915Z [RUSTC-TIMING] glob test:false 1.925
2019-11-13T18:18:38.6571415Z error: build failed
2019-11-13T18:18:38.6617757Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2019-11-13T18:18:38.6619389Z expected success, got: exit code: 101
2019-11-13T18:18:38.6619389Z expected success, got: exit code: 101
2019-11-13T18:18:38.7636655Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-13T18:18:38.7637098Z Build completed unsuccessfully in 1:49:45
2019-11-13T18:18:38.7886708Z == clock drift check ==
2019-11-13T18:18:38.8796453Z   local time: Wed Nov 13 18:18:38 CUT 2019
2019-11-13T18:18:39.3940811Z   network time: Wed, 13 Nov 2019 18:18:39 GMT
2019-11-13T18:18:39.3952820Z == end clock drift check ==
2019-11-13T18:18:39.4831406Z 
2019-11-13T18:18:39.8868888Z ##[error]Bash exited with code '1'.
2019-11-13T18:18:39.9415037Z ##[section]Starting: Checkout
2019-11-13T18:18:40.0229478Z ==============================================================================
2019-11-13T18:18:40.0229643Z Task         : Get sources
2019-11-13T18:18:40.0229720Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
