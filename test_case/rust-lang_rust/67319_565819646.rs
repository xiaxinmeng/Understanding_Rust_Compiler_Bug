plain
2019-12-15T12:26:15.2753950Z 
2019-12-15T12:26:15.2754513Z 
2019-12-15T12:26:15.2754982Z 
2019-12-15T12:26:15.2755577Z 
2019-12-15T12:26:15.2755775Z  - #65778 (Stabilize `std::{rc,sync}::Weak::{weak_count, strong_count}`)
2019-12-15T12:26:15.2756060Z  - #67206 (Update cargo, books)
2019-12-15T12:26:15.2756213Z  - #67285 (Indicate origin of where type parameter for uninferred types )
2019-12-15T12:26:15.2757496Z  - #67317 (fix type_name_of_val doc comment)
2019-12-15T12:26:15.2762316Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-15T12:26:15.2762987Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-15T12:26:15.2763233Z AGENT_HOMEDIRECTORY=C:\agents\2.163.1
2019-12-15T12:26:15.2763433Z AGENT_ID=520
---
2019-12-15T15:28:24.8186850Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-15T15:28:24.8186941Z    |
2019-12-15T15:28:24.8187078Z    = note: perhaps that crate needs to be recompiled?
2019-12-15T15:28:24.8187216Z    = note: the following crate versions were found:
2019-12-15T15:28:24.8187399Z            crate `rustc_plugin_impl`: \\?\C:\MORE_SPACE\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\librustc_plugin_impl-ab0921028d5d8ffa.rlib
2019-12-15T15:28:24.8187602Z            crate `rustc_driver`: \\?\C:\MORE_SPACE\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\rustc_driver-79b9dd467afe5b7b.dll
2019-12-15T15:28:24.8187797Z error: aborting due to previous error
2019-12-15T15:28:24.8187859Z 
2019-12-15T15:28:24.8242040Z [RUSTC-TIMING] rustdoc test:false 0.338
2019-12-15T15:28:24.8260193Z error: could not compile `rustdoc`.
2019-12-15T15:28:24.8260193Z error: could not compile `rustdoc`.
2019-12-15T15:28:24.8272660Z warning: build failed, waiting for other jobs to finish...
2019-12-15T15:28:27.9085634Z [RUSTC-TIMING] serde_json test:false 5.696
2019-12-15T15:28:27.9254930Z error: build failed
2019-12-15T15:28:27.9289718Z 
2019-12-15T15:28:27.9289906Z 
2019-12-15T15:28:27.9290110Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "-Zconfig-profile" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/rustdoc\\Cargo.toml" "-p" "rustdoc:0.0.0" "--"
2019-12-15T15:28:27.9290739Z 
2019-12-15T15:28:27.9290771Z 
2019-12-15T15:28:27.9290771Z 
2019-12-15T15:28:28.0353013Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2019-12-15T15:28:28.0353178Z Build completed unsuccessfully in 2:50:30
2019-12-15T15:28:28.0941413Z make: *** [Makefile:80: ci-subset-1] Error 1
2019-12-15T15:28:28.1628256Z   local time: Sun Dec 15 15:28:28 CUT 2019
2019-12-15T15:28:28.5592478Z   network time: Sun, 15 Dec 2019 15:28:28 GMT
2019-12-15T15:28:28.5613990Z == end clock drift check ==
2019-12-15T15:28:28.6413209Z 
2019-12-15T15:28:28.6413209Z 
2019-12-15T15:28:29.0555280Z ##[error]Bash exited with code '2'.
2019-12-15T15:28:29.1323271Z ##[section]Starting: Checkout
2019-12-15T15:28:29.2378711Z ==============================================================================
2019-12-15T15:28:29.2378859Z Task         : Get sources
2019-12-15T15:28:29.2378951Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
