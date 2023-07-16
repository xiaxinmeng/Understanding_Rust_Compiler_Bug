plain
2019-08-01T05:00:18.8190651Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T05:00:18.8191200Z 
2019-08-01T05:00:18.8191647Z   git checkout -b <new-branch-name>
2019-08-01T05:00:18.8192067Z 
2019-08-01T05:00:18.8192438Z HEAD is now at e6bb16fa3 Auto merge of #63182 - Centril:rollup-bjlnael, r=Centril
2019-08-01T05:00:18.8620078Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T05:00:18.8731824Z ==============================================================================
2019-08-01T05:00:18.8731927Z Task         : Bash
2019-08-01T05:00:18.8732019Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T07:23:50.3963787Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-01T07:23:50.3963857Z 
2019-08-01T07:23:50.4359505Z testing https://github.com/servo/servo
2019-08-01T07:23:50.5652175Z Initialized empty Git repository in D:/a/1/s/build/ct/servo/.git/
2019-08-01T07:23:50.5904875Z fatal: Could not parse object '9043f247d9b031ed285e880e4b90aa523d4a63ae'.
2019-08-01T07:24:02.4987282Z  * branch                master     -> FETCH_HEAD
2019-08-01T07:24:02.4987282Z  * branch                master     -> FETCH_HEAD
2019-08-01T07:24:02.5346460Z fatal: Could not parse object '9043f247d9b031ed285e880e4b90aa523d4a63ae'.
2019-08-01T07:24:08.2033828Z  * branch                master     -> FETCH_HEAD
2019-08-01T07:24:09.4770594Z Checking out files:   1% (1466/84814)   
2019-08-01T07:24:09.9110370Z Checking out files:   2% (1697/84814)   
2019-08-01T07:24:10.5220829Z Checking out files:   3% (2545/84814)   
---
2019-08-01T07:25:18.8435549Z Checking out files:  98% (83453/84814)   
2019-08-01T07:25:19.3919173Z Checking out files:  99% (83966/84814)   
2019-08-01T07:25:19.3924356Z Checking out files: 100% (84814/84814)   
2019-08-01T07:25:19.3924478Z Checking out files: 100% (84814/84814), done.
2019-08-01T07:25:19.6680666Z HEAD is now at 9043f247d Auto merge of #23839 - Manishearth:end-improve, r=asajeffrey
2019-08-01T07:25:24.2034068Z     Updating git repository `https://github.com/alexcrichton/cmake-rs`
2019-08-01T07:25:24.7848278Z     Updating git repository `https://github.com/servo/iovec.git`
2019-08-01T07:25:25.0257272Z     Updating git repository `https://github.com/servo/mio.git`
2019-08-01T07:25:35.7855929Z     Updating git repository `https://github.com/servo/webxr`
2019-08-01T07:25:36.5018451Z     Updating git repository `https://github.com/servo/webrender`
2019-08-01T07:25:42.7843703Z     Updating git repository `https://github.com/servo/gaol`
2019-08-01T07:25:43.7371642Z     Updating git repository `https://github.com/servo/devices`
2019-08-01T07:25:43.7371642Z     Updating git repository `https://github.com/servo/devices`
2019-08-01T07:25:44.0213148Z     Updating git repository `https://github.com/servo/rust-azure`
2019-08-01T07:25:44.9322630Z     Updating git repository `https://github.com/jrmuizel/raqote`
2019-08-01T07:25:45.4193693Z     Updating git repository `https://github.com/pcwalton/signpost.git`
2019-08-01T07:25:45.6094776Z     Updating git repository `https://github.com/servo/fontsan`
2019-08-01T07:25:58.2738686Z     Updating git repository `https://github.com/servo/rust-mozjs`
2019-08-01T07:25:59.0914886Z     Updating git repository `https://github.com/energymon/energymon-sys.git`
---
2019-08-01T07:27:25.8620899Z   Downloaded ident_case v1.0.0
2019-08-01T07:27:26.1277250Z    Compiling winapi v0.3.7
2019-08-01T07:27:26.1277809Z    Compiling autocfg v0.1.4
2019-08-01T07:27:27.6560877Z    Compiling proc-macro2 v0.4.26
2019-08-01T07:27:27.9889662Z error: linker `lld-link.exe` not found
2019-08-01T07:27:27.9902349Z   = note: The system cannot find the file specified. (os error 2)
2019-08-01T07:27:27.9902736Z 
2019-08-01T07:27:27.9902736Z 
2019-08-01T07:27:27.9903043Z note: the msvc targets depend on the msvc linker but `link.exe` was not found
2019-08-01T07:27:27.9903384Z 
2019-08-01T07:27:27.9903507Z note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option
2019-08-01T07:27:28.0078423Z error: aborting due to previous error
2019-08-01T07:27:28.0079682Z 
2019-08-01T07:27:28.0079682Z 
2019-08-01T07:27:28.0321149Z error: Could not compile `winapi`.
2019-08-01T07:27:28.0321328Z warning: build failed, waiting for other jobs to finish...
2019-08-01T07:27:28.2455967Z error: linker `lld-link.exe` not found
2019-08-01T07:27:28.2457414Z   = note: The system cannot find the file specified. (os error 2)
2019-08-01T07:27:28.2457660Z 
2019-08-01T07:27:28.2457660Z 
2019-08-01T07:27:28.2457927Z note: the msvc targets depend on the msvc linker but `link.exe` was not found
2019-08-01T07:27:28.2458069Z 
2019-08-01T07:27:28.2458256Z note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option
2019-08-01T07:27:28.2464613Z error: aborting due to previous error
2019-08-01T07:27:28.2464844Z 
2019-08-01T07:27:28.2885128Z error: Could not compile `proc-macro2`.
2019-08-01T07:27:28.2885592Z 
2019-08-01T07:27:28.2885592Z 
2019-08-01T07:27:28.2885805Z To learn more, run the command again with --verbose.
2019-08-01T07:27:28.3265434Z thread 'main' panicked at 'tests failed for https://github.com/servo/servo', src\tools\cargotest\main.rs:86:9
2019-08-01T07:27:28.3307701Z 
2019-08-01T07:27:28.3308666Z 
2019-08-01T07:27:28.3308933Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\cargotest.exe" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "D:\\a\\1\\s\\build\\ct"
2019-08-01T07:27:28.3309119Z expected success, got: exit code: 101
2019-08-01T07:27:28.3309119Z expected success, got: exit code: 101
2019-08-01T07:27:28.3309172Z 
2019-08-01T07:27:28.3309234Z 
2019-08-01T07:27:28.4675984Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/tools/cargotest src/tools/cargo
2019-08-01T07:27:28.4677215Z Build completed unsuccessfully in 2:15:21
2019-08-01T07:27:29.0530413Z ##[error]Bash exited with code '1'.
2019-08-01T07:27:29.1402453Z ##[section]Starting: Upload CPU usage statistics
2019-08-01T07:27:29.4331318Z ==============================================================================
2019-08-01T07:27:29.4331430Z Task         : Bash
2019-08-01T07:27:29.4331531Z Description  : Run a Bash script on macOS, Linux, or Windows
