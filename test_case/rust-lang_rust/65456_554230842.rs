plain
2019-11-15T04:32:58.2560739Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-11-15T04:32:58.2560822Z SYSTEM_TIMELINEID=c3b1109c-2628-4bfe-842b-a8877906334f
2019-11-15T04:32:58.2560908Z SYSTEM_TOTALJOBSINPHASE=16
2019-11-15T04:32:58.2560972Z SYSTEM_WORKFOLDER=D:\a
2019-11-15T04:32:58.2561070Z Suggest borrowing when it would satisfy an unmet trait bound
2019-11-15T04:32:58.2561395Z TEMP=/tmp
2019-11-15T04:32:58.2561468Z TERM=cygwin
2019-11-15T04:32:58.2561525Z TF_BUILD=True
2019-11-15T04:32:58.2561601Z TMP=/tmp
---
2019-11-15T04:32:58.2563007Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-15T04:32:58.2563107Z When there are multiple implementors for the same trait that is present
2019-11-15T04:32:58.2563196Z _=/usr/bin/printenv
2019-11-15T04:32:58.2563258Z agent.jobstatus=Succeeded
2019-11-15T04:32:58.2563361Z in an unmet binding, modify the E0277 error to refer to the parent
2019-11-15T04:32:58.2563450Z obligation and verify whether borrowing the argument being passed in
2019-11-15T04:32:58.2563553Z would satisfy the unmet bound. If it would, suggest it.
2019-11-15T04:32:58.2563719Z disk usage:
2019-11-15T04:32:58.3336173Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-15T04:32:58.3336370Z C:/Program Files/Git  256G  140G  116G  55% /
2019-11-15T04:32:58.3336885Z D:                     14G  2.0G   13G  15% /d
---
2019-11-15T06:23:58.6001655Z [RUSTC-TIMING] hex test:false 3.158
2019-11-15T06:23:58.6064239Z    Compiling lazycell v1.2.1
2019-11-15T06:23:58.7764696Z [RUSTC-TIMING] lazycell test:false 0.153
2019-11-15T06:23:58.8193806Z    Compiling glob v0.3.0
2019-11-15T06:23:59.0564140Z memory allocation of 2147483656 bytes failed[RUSTC-TIMING] hex test:false 4.036
2019-11-15T06:23:59.0598870Z error: could not compile `hex`.
2019-11-15T06:23:59.0599051Z Caused by:
2019-11-15T06:23:59.0599051Z Caused by:
2019-11-15T06:23:59.0599422Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.3.2\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 -C metadata=0e6de99ea4fd2f60 -C extra-filename=-0e6de99ea4fd2f60 --out-dir D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps --target x86_64-pc-windows-gnu -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-15T06:24:00.7216402Z [RUSTC-TIMING] glob test:false 1.885
2019-11-15T06:24:00.7374531Z error: build failed
2019-11-15T06:24:00.7415777Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2019-11-15T06:24:00.7416452Z expected success, got: exit code: 101
2019-11-15T06:24:00.7416452Z expected success, got: exit code: 101
2019-11-15T06:24:00.8595437Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-15T06:24:00.8595685Z Build completed unsuccessfully in 1:42:29
2019-11-15T06:24:00.8984279Z == clock drift check ==
2019-11-15T06:24:01.0054246Z   local time: Fri Nov 15 06:24:01 CUT 2019
2019-11-15T06:24:01.5402766Z   network time: Fri, 15 Nov 2019 06:24:01 GMT
2019-11-15T06:24:01.5420822Z == end clock drift check ==
2019-11-15T06:24:01.6202252Z 
2019-11-15T06:24:02.0347598Z ##[error]Bash exited with code '1'.
2019-11-15T06:24:02.0903356Z ##[section]Starting: Checkout
2019-11-15T06:24:02.1566370Z ==============================================================================
2019-11-15T06:24:02.1566516Z Task         : Get sources
2019-11-15T06:24:02.1566601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
