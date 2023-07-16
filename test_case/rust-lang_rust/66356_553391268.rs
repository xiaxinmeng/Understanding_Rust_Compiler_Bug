plain
2019-11-13T11:00:13.2341678Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-13T11:00:13.2342156Z 
2019-11-13T11:00:13.2342615Z   git checkout -b <new-branch-name>
2019-11-13T11:00:13.2343166Z 
2019-11-13T11:00:13.2343529Z HEAD is now at 688a74bc3 Auto merge of #66356 - JohnTitor:rollup-1mh7jsr, r=JohnTitor
2019-11-13T11:00:13.2883198Z ##[section]Starting: Decide whether to run this job
2019-11-13T11:00:13.2988816Z ==============================================================================
2019-11-13T11:00:13.2988910Z Task         : Bash
2019-11-13T11:00:13.2989007Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-13T11:00:14.6036261Z 
2019-11-13T11:00:14.6036401Z 
2019-11-13T11:00:14.6036524Z 
2019-11-13T11:00:14.6036823Z 
2019-11-13T11:00:14.6037022Z  - #65932 (download .tar.xz if python3 is used)
2019-11-13T11:00:14.6037235Z  - #66074 ([mir-opt] Turn on the `ConstProp` pass by default)
2019-11-13T11:00:14.6038048Z  - #66094 (Fix documentation for `Iterator::count()`.)
2019-11-13T11:00:14.6038359Z  - #66166 (rename cfg(rustdoc) into cfg(doc))
2019-11-13T11:00:14.6038560Z  - #66227 (docs: Fix link to BufWriter::flush)
2019-11-13T11:00:14.6038751Z  - #66292 (add Result::map_or)
2019-11-13T11:00:14.6038935Z  - #66297 (Add a callback that allows compiler consumers to override queries.)
2019-11-13T11:00:14.6039161Z  - #66317 (Use a relative bindir for rustdoc to find rustc)
2019-11-13T11:00:14.6039397Z  - #66330 (Improve non-exhaustiveness handling in usefulness checking)
2019-11-13T11:00:14.6040162Z  - #66334 (Move Session fields to CrateStore)
2019-11-13T11:00:14.6043678Z  - #66335 (Move self-profile infrastructure to data structures)
2019-11-13T11:00:14.6043678Z  - #66335 (Move self-profile infrastructure to data structures)
2019-11-13T11:00:14.6048089Z  - #66337 (Remove dead code for encoding/decoding lint IDs)
2019-11-13T11:00:14.6067626Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-13T11:00:14.6068132Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-13T11:00:14.6068332Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-13T11:00:14.6068505Z AGENT_ID=509
---
2019-11-13T11:00:14.6086098Z BUILD_SOURCEBRANCHNAME=auto
2019-11-13T11:00:14.6086262Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-13T11:00:14.6086474Z BUILD_SOURCEVERSION=688a74bc3317dfa642732205e7eeecdd0590fb34
2019-11-13T11:00:14.6086666Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-13T11:00:14.6111577Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66356 - JohnTitor:rollup-1mh7jsr, r=JohnTitor
2019-11-13T11:00:14.6267209Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-13T11:00:14.6269102Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-13T11:00:14.6269228Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-13T11:00:14.6269305Z COMPUTERNAME=fv-az433
---
2019-11-13T12:51:08.9013053Z [RUSTC-TIMING] vec_map test:false 0.304
2019-11-13T12:51:08.9064305Z    Compiling hex v0.3.2
2019-11-13T12:51:09.1056234Z [RUSTC-TIMING] semver_parser test:false 2.285
2019-11-13T12:51:09.1134450Z    Compiling hex v0.4.0
2019-11-13T12:51:12.7602993Z memory allocation of 4294967304 bytes failed[RUSTC-TIMING] hex test:false 3.624
2019-11-13T12:51:12.7604424Z error: could not compile `hex`.
2019-11-13T12:51:12.7605225Z Caused by:
2019-11-13T12:51:12.7605225Z Caused by:
2019-11-13T12:51:12.7605749Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --edition=2018 --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.4.0\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 --cfg "feature=\"default\"" --cfg "feature=\"std\"" -C metadata=b3cb7a0e06fa5820 -C extra-filename=-b3cb7a0e06fa5820 --out-dir D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps --target x86_64-pc-windows-gnu -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-13T12:51:12.7606573Z warning: build failed, waiting for other jobs to finish...
2019-11-13T12:51:12.7853194Z memory allocation of 536870920 bytes failed[RUSTC-TIMING] hex test:false 3.862
2019-11-13T12:51:12.8015107Z error: could not compile `hex`.
2019-11-13T12:51:12.8015825Z Caused by:
2019-11-13T12:51:12.8015825Z Caused by:
2019-11-13T12:51:12.8016371Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.3.2\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 -C metadata=c9ae03471b0a6bfa -C extra-filename=-c9ae03471b0a6bfa --out-dir D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps --target x86_64-pc-windows-gnu -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\x86_64-pc-windows-gnu\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-gnu\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-13T12:51:12.8054298Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2019-11-13T12:51:12.9533696Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-13T12:51:12.9533930Z Build completed unsuccessfully in 1:41:58
2019-11-13T12:51:12.9901212Z == clock drift check ==
2019-11-13T12:51:13.0848028Z   local time: Wed Nov 13 12:51:13 CUT 2019
2019-11-13T12:51:13.0848028Z   local time: Wed Nov 13 12:51:13 CUT 2019
2019-11-13T12:51:13.5543018Z   network time: Wed, 13 Nov 2019 12:51:13 GMT
2019-11-13T12:51:13.5551469Z == end clock drift check ==
2019-11-13T12:51:13.6204686Z 
2019-11-13T12:51:13.9576848Z ##[error]Bash exited with code '1'.
2019-11-13T12:51:14.0368445Z ##[section]Starting: Checkout
2019-11-13T12:51:14.1319249Z ==============================================================================
2019-11-13T12:51:14.1319368Z Task         : Get sources
2019-11-13T12:51:14.1319466Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
