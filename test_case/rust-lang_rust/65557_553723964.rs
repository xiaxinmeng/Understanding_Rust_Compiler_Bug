plain
2019-11-14T02:49:00.1568845Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-14T02:49:00.1569091Z 
2019-11-14T02:49:00.1569249Z   git checkout -b <new-branch-name>
2019-11-14T02:49:00.1569369Z 
2019-11-14T02:49:00.1569712Z HEAD is now at e360fccf5 Auto merge of #65557 - haraldh:error_iter_rename, r=sfackler
2019-11-14T02:49:00.2079924Z ##[section]Starting: Decide whether to run this job
2019-11-14T02:49:00.2222522Z ==============================================================================
2019-11-14T02:49:00.2222633Z Task         : Bash
2019-11-14T02:49:00.2222879Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-14T02:49:01.6249241Z 
2019-11-14T02:49:01.6249357Z 
2019-11-14T02:49:01.6250363Z 
2019-11-14T02:49:01.6251211Z 
2019-11-14T02:49:01.6251387Z    1. Such iterators are helpful. They should better be stabilized sooner than later.
2019-11-14T02:49:01.6251551Z    1. The chosen name should be telling and reflect the fact that self is included. `.chained()` was chosen in honor of error-chain and because the iterator iterates over the chain of errors that is somehow included in self.
2019-11-14T02:49:01.6251769Z    1. The resulting iterator is named `Chain` because the `error::Chain` is what we want to have.
2019-11-14T02:49:01.6251924Z    1. self should be included. It is easy to .skip(1) it.  Not including self is harmful because it is harder to add self to the iterator than to remove it.
2019-11-14T02:49:01.6252029Z * Error::iter_chain() -> Error::chained()
2019-11-14T02:49:01.6252121Z * ErrorIter -> Chain
2019-11-14T02:49:01.6252198Z * ~~Error::iter_chain() -> Error::chained()~~
2019-11-14T02:49:01.6252297Z * ~~Error::iter_sources() -> Error::ancestors()~~
2019-11-14T02:49:01.6252444Z * ~~ErrorIter -> Chained and Ancestors~~
2019-11-14T02:49:01.6252808Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-14T02:49:01.6252993Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-14T02:49:01.6253185Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-14T02:49:01.6253341Z AGENT_ID=511
---
2019-11-14T02:49:01.6260601Z BUILD_SOURCEBRANCHNAME=auto
2019-11-14T02:49:01.6260702Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-14T02:49:01.6260792Z BUILD_SOURCEVERSION=e360fccf50220fab2a8e0897fe1006f12f8e7630
2019-11-14T02:49:01.6260944Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-14T02:49:01.6261044Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #65557 - haraldh:error_iter_rename, r=sfackler
2019-11-14T02:49:01.6261230Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-14T02:49:01.6261332Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-14T02:49:01.6261747Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-14T02:49:01.6261820Z COMPUTERNAME=fv-az433
---
2019-11-14T02:49:01.6281888Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-11-14T02:49:01.6282151Z SYSTEM_TIMELINEID=0bea1525-3c9c-4851-b434-47e2ba279c26
2019-11-14T02:49:01.6282232Z SYSTEM_TOTALJOBSINPHASE=16
2019-11-14T02:49:01.6282315Z SYSTEM_WORKFOLDER=D:\a
2019-11-14T02:49:01.6282435Z So, it seems, that even Path::ancestors() includes itself. So, to avoid confusion and simplify it more, I reduced PR  #65557 to only have `chained` and `Chain`.
2019-11-14T02:49:01.6282679Z TEMP=/tmp
2019-11-14T02:49:01.6282757Z TERM=cygwin
2019-11-14T02:49:01.6282835Z TF_BUILD=True
2019-11-14T02:49:01.6282897Z TMP=/tmp
---
2019-11-14T02:49:01.6284439Z according to
2019-11-14T02:49:01.6284501Z agent.jobstatus=Succeeded
2019-11-14T02:49:01.6284630Z https://github.com/rust-lang/rust/issues/58520
2019-11-14T02:49:01.6284749Z https://github.com/rust-lang/rust/issues/58520#issuecomment-527704110
2019-11-14T02:49:01.6285419Z rename Error::iter_chain() and remove Error::iter_sources()
2019-11-14T02:49:01.6285499Z ~~Rename~~
2019-11-14T02:49:01.6285653Z disk usage:
2019-11-14T02:49:01.6943015Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-14T02:49:01.6949544Z C:/Program Files/Git  256G  140G  116G  55% /
2019-11-14T02:49:01.6949735Z D:                     14G  2.0G   13G  15% /d
---
2019-11-14T04:53:05.7237824Z [RUSTC-TIMING] semver_parser test:false 2.541
2019-11-14T04:53:05.7318274Z    Compiling hex v0.4.0
2019-11-14T04:53:09.7813556Z [RUSTC-TIMING] hex test:false 4.194
2019-11-14T04:53:09.8460434Z    Compiling bytesize v1.0.0
2019-11-14T04:53:10.0604070Z memory allocation of 2147483656 bytes failed[RUSTC-TIMING] hex test:false 4.323
2019-11-14T04:53:10.0641035Z error: could not compile `hex`.
2019-11-14T04:53:10.0642112Z Caused by:
2019-11-14T04:53:10.0642112Z Caused by:
2019-11-14T04:53:10.0643424Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --edition=2018 --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.4.0\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 --cfg "feature=\"default\"" --cfg "feature=\"std\"" -C metadata=e4be2a5fb99e25f1 -C extra-filename=-e4be2a5fb99e25f1 --out-dir D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\x86_64-pc-windows-msvc\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Ctarget-feature=+crt-static -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-14T04:53:10.1262422Z [RUSTC-TIMING] bytesize test:false 0.270
2019-11-14T04:53:10.1548573Z error: build failed
2019-11-14T04:53:10.1604892Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2019-11-14T04:53:10.1605424Z expected success, got: exit code: 101
2019-11-14T04:53:10.1605424Z expected success, got: exit code: 101
2019-11-14T04:53:10.2684409Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-14T04:53:10.2684808Z Build completed unsuccessfully in 1:53:22
2019-11-14T04:53:10.3536599Z == clock drift check ==
2019-11-14T04:53:10.4649895Z   local time: Thu Nov 14 04:53:10 CUT 2019
2019-11-14T04:53:10.8310692Z   network time: Thu, 14 Nov 2019 04:53:10 GMT
2019-11-14T04:53:10.8370536Z == end clock drift check ==
2019-11-14T04:53:10.9081031Z 
2019-11-14T04:53:11.2547952Z ##[error]Bash exited with code '1'.
2019-11-14T04:53:11.3469449Z ##[section]Starting: Checkout
2019-11-14T04:53:11.4278646Z ==============================================================================
2019-11-14T04:53:11.4278806Z Task         : Get sources
2019-11-14T04:53:11.4278916Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
