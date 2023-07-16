plain
2020-01-05T04:46:59.8670852Z do so (now or later) by using -b with the checkout command again. Example:
2020-01-05T04:46:59.8671136Z 
2020-01-05T04:46:59.8672440Z   git checkout -b <new-branch-name>
2020-01-05T04:46:59.8672887Z 
2020-01-05T04:46:59.8673334Z HEAD is now at db438051e Auto merge of #67770 - Centril:reduce-diversity-2, r=petrochenkov
2020-01-05T04:46:59.9035754Z ##[section]Starting: Setup environment
2020-01-05T04:46:59.9135123Z ==============================================================================
2020-01-05T04:46:59.9135201Z Task         : Bash
2020-01-05T04:46:59.9135282Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2020-01-05T07:34:02.0191795Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-05T07:34:02.0191867Z    |
2020-01-05T07:34:02.0191983Z    = note: perhaps that crate needs to be recompiled?
2020-01-05T07:34:02.0192099Z    = note: the following crate versions were found:
2020-01-05T07:34:02.0192253Z            crate `rustc_plugin_impl`: \\?\C:\MORE_SPACE\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\librustc_plugin_impl-9db50e323f76fa0d.rlib
2020-01-05T07:34:02.0200354Z            crate `rustc_driver`: \\?\C:\MORE_SPACE\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\rustc_driver-bf01c9934c887e52.dll
2020-01-05T07:34:02.0200540Z error: aborting due to previous error
2020-01-05T07:34:02.0200879Z 
2020-01-05T07:34:02.0253000Z [RUSTC-TIMING] rustdoc test:false 0.364
2020-01-05T07:34:02.0286652Z error: could not compile `rustdoc`.
2020-01-05T07:34:02.0286652Z error: could not compile `rustdoc`.
2020-01-05T07:34:02.0568026Z warning: build failed, waiting for other jobs to finish...
2020-01-05T07:34:05.0380847Z [RUSTC-TIMING] serde_json test:false 5.499
2020-01-05T07:34:05.0527989Z error: build failed
2020-01-05T07:34:05.0563228Z 
2020-01-05T07:34:05.0563380Z 
2020-01-05T07:34:05.0563543Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "-Zconfig-profile" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/rustdoc\\Cargo.toml" "-p" "rustdoc:0.0.0" "--"
2020-01-05T07:34:05.0563740Z 
2020-01-05T07:34:05.0563806Z 
2020-01-05T07:34:05.0563806Z 
2020-01-05T07:34:05.1446261Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2020-01-05T07:34:05.1447046Z Build completed unsuccessfully in 2:35:39
2020-01-05T07:34:05.2169667Z make: *** [Makefile:80: ci-subset-1] Error 1
2020-01-05T07:34:05.2799857Z   local time: Sun Jan  5 07:34:05 CUT 2020
2020-01-05T07:34:05.7069456Z   network time: Sun, 05 Jan 2020 07:34:05 GMT
2020-01-05T07:34:05.7103186Z == end clock drift check ==
2020-01-05T07:34:05.8079211Z 
2020-01-05T07:34:05.8079211Z 
2020-01-05T07:34:06.0942127Z ##[error]Bash exited with code '2'.
2020-01-05T07:34:06.1869258Z ##[section]Starting: Checkout
2020-01-05T07:34:06.2776363Z ==============================================================================
2020-01-05T07:34:06.2776492Z Task         : Get sources
2020-01-05T07:34:06.2776580Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
