plain
2019-12-30T11:53:56.6919804Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T11:53:56.7595087Z ##[command]git config gc.auto 0
2019-12-30T11:53:56.8262776Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T11:53:56.8626102Z ##[command]git config --get-all http.proxy
2019-12-30T11:53:56.9044798Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67711/merge:refs/remotes/pull/67711/merge
---
2019-12-30T11:54:39.6818536Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-30T11:54:39.6822777Z 
2019-12-30T11:54:39.6823553Z   git checkout -b <new-branch-name>
2019-12-30T11:54:39.6823627Z 
2019-12-30T11:54:39.6823713Z HEAD is now at 4236d9938 Merge 508c3e2d64af3652d5447c0f488a34765f6324b2 into 580ac0b4f1c6f9cf76f6edafdaf9806437770aff
2019-12-30T11:54:39.7243432Z ##[section]Starting: Setup environment
2019-12-30T11:54:39.7500134Z ==============================================================================
2019-12-30T11:54:39.7500191Z Task         : Bash
2019-12-30T11:54:39.7500397Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-30T11:54:41.8792480Z BUILD_SOURCEBRANCH=refs/pull/67711/merge
2019-12-30T11:54:41.8792627Z BUILD_SOURCEBRANCHNAME=merge
2019-12-30T11:54:41.8792674Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-30T11:54:41.8792721Z BUILD_SOURCEVERSION=4236d9938ac349522fef889ab54528a38453555e
2019-12-30T11:54:41.8792765Z BUILD_SOURCEVERSIONAUTHOR=Amanieu d'Antras
2019-12-30T11:54:41.8793472Z CI_JOB_NAME=i686-msvc-1
2019-12-30T11:54:41.8793533Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-30T11:54:41.8793608Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-30T11:54:41.8793661Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-30T11:54:41.8811569Z SYSTEM_PHASEID=27eddb93-7805-576c-c80f-37b2176e40f7
2019-12-30T11:54:41.8811609Z SYSTEM_PHASENAME=Windows
2019-12-30T11:54:41.8811672Z SYSTEM_PIPELINESTARTTIME=2019-12-30 11:53:46+00:00
2019-12-30T11:54:41.8811722Z SYSTEM_PLANID=5c5fa72e-48b5-4dcf-a8e4-760aee4bc812
2019-12-30T11:54:41.8811762Z SYSTEM_PULLREQUEST_ISFORK=True
2019-12-30T11:54:41.8811819Z SYSTEM_PULLREQUEST_MERGEDAT=
2019-12-30T11:54:41.8811860Z SYSTEM_PULLREQUEST_PULLREQUESTID=357878796
2019-12-30T11:54:41.8812067Z SYSTEM_PULLREQUEST_PULLREQUESTNUMBER=67711
2019-12-30T11:54:41.8812109Z SYSTEM_PULLREQUEST_SOURCEBRANCH=fix_unwind_leak
2019-12-30T11:54:41.8812173Z SYSTEM_PULLREQUEST_SOURCECOMMITID=508c3e2d64af3652d5447c0f488a34765f6324b2
2019-12-30T11:54:41.8812265Z SYSTEM_PULLREQUEST_SOURCEREPOSITORYURI=***.git
2019-12-30T11:54:41.8812325Z SYSTEM_PULLREQUEST_TARGETBRANCH=master
2019-12-30T11:54:41.8812370Z SYSTEM_RESTRICTSECRETS=True
2019-12-30T11:54:41.8812445Z SYSTEM_STAGEATTEMPT=1
2019-12-30T11:54:41.8812504Z SYSTEM_STAGEDISPLAYNAME=__default
2019-12-30T11:54:41.8812545Z SYSTEM_STAGEID=96ac2280-8cb4-5df5-99de-dd2da759617d
2019-12-30T11:54:41.8812591Z SYSTEM_STAGENAME=__default
---
2019-12-30T12:00:55.7329875Z + modules=($modules)
2019-12-30T12:00:55.7329914Z + use_git=
2019-12-30T12:00:55.7525786Z ++ git config --file .gitmodules --get-regexp '\.url$'
2019-12-30T12:00:55.7614932Z ++ cut '-d ' -f2
2019-12-30T12:00:55.7805135Z + urls='***-installer.git
2019-12-30T12:00:55.7817715Z https://github.com/rust-lang/cargo.git
2019-12-30T12:00:55.7817948Z https://github.com/rust-lang/reference.git
2019-12-30T12:00:55.7818115Z https://github.com/rust-lang/book.git
2019-12-30T12:00:55.7818307Z https://github.com/rust-lang-nursery/rls.git
---
2019-12-30T12:00:56.1671674Z ++ awk '{print $3}'
2019-12-30T12:00:56.1822406Z + commit=b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4
2019-12-30T12:00:56.1834479Z + git rm src/doc/rust-by-example
2019-12-30T12:00:56.2718753Z rm 'src/doc/rust-by-example'
2019-12-30T12:00:56.2979106Z + url=***-by-example.git
2019-12-30T12:00:56.2979400Z + url=***-by-example
2019-12-30T12:00:56.3156930Z + fetch_github_commit_archive src/doc/rust-by-example ***-by-example/archive/b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4.tar.gz
2019-12-30T12:00:56.3277847Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-12-30T12:00:56.3277847Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-12-30T12:00:56.3278180Z + retry sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4.tar.gz'
2019-12-30T12:00:56.3278478Z + echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4.tar.gz'
2019-12-30T12:00:56.3278747Z Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4.tar.gz
2019-12-30T12:00:56.3279185Z + local max=5
2019-12-30T12:00:56.3279314Z + true
2019-12-30T12:00:56.3279314Z + true
2019-12-30T12:00:56.3279542Z + sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/b7ac1bc76b7d02a43c83b3a931d226f708aa1ff4.tar.gz'
2019-12-30T12:00:56.3279888Z + for i in ${!modules[@]}
2019-12-30T12:00:56.3280019Z + module=src/stdarch
2019-12-30T12:00:56.3280227Z + [[  src/llvm-project src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\s\t\d\a\r\c\h\ * ]]
2019-12-30T12:00:56.3280419Z + use_git=' src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdarch'
---
2019-12-30T12:00:59.9694478Z Submodule 'src/doc/edition-guide' (https://github.com/rust-lang/edition-guide.git) registered for path 'src/doc/edition-guide'
2019-12-30T12:00:59.9694650Z Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
2019-12-30T12:00:59.9694728Z Submodule 'src/doc/nomicon' (https://github.com/rust-lang/nomicon.git) registered for path 'src/doc/nomicon'
2019-12-30T12:00:59.9694825Z Submodule 'src/doc/reference' (https://github.com/rust-lang/reference.git) registered for path 'src/doc/reference'
2019-12-30T12:00:59.9695005Z Submodule 'src/doc/rustc-guide' (***c-guide.git) registered for path 'src/doc/rustc-guide'
2019-12-30T12:00:59.9695212Z Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
2019-12-30T12:00:59.9695284Z Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
2019-12-30T12:00:59.9695373Z Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
2019-12-30T12:00:59.9695441Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-12-30T12:00:59.9695441Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-12-30T12:00:59.9696471Z Submodule 'src/rust-installer' (***-installer.git) registered for path 'src/tools/rust-installer'
2019-12-30T12:01:00.1534051Z Cloning into 'D:/a/1/s/src/doc/edition-guide'...
2019-12-30T12:01:18.3850241Z + break
2019-12-30T12:01:18.6198297Z + mkdir src/llvm-project
2019-12-30T12:01:19.0399974Z + touch src/llvm-project/.git
---
2019-12-30T12:02:13.5990744Z file:.git/config submodule.src/doc/nomicon.url=https://github.com/rust-lang/nomicon.git
2019-12-30T12:02:13.5990882Z file:.git/config submodule.src/doc/reference.active=true
2019-12-30T12:02:13.5991035Z file:.git/config submodule.src/doc/reference.url=https://github.com/rust-lang/reference.git
2019-12-30T12:02:13.5992324Z file:.git/config submodule.src/doc/rustc-guide.active=true
2019-12-30T12:02:13.5992836Z file:.git/config submodule.src/doc/rustc-guide.url=***c-guide.git
2019-12-30T12:02:13.5993191Z file:.git/config submodule.src/stdarch.url=https://github.com/rust-lang/stdarch.git
2019-12-30T12:02:13.5993357Z file:.git/config submodule.src/tools/cargo.active=true
2019-12-30T12:02:13.5993597Z file:.git/config submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-12-30T12:02:13.5993977Z file:.git/config submodule.src/tools/clippy.active=true
2019-12-30T12:02:13.5993977Z file:.git/config submodule.src/tools/clippy.active=true
2019-12-30T12:02:13.5994661Z file:.git/config submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-12-30T12:02:13.5994937Z file:.git/config submodule.src/tools/miri.active=true
2019-12-30T12:02:13.5995141Z file:.git/config submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-12-30T12:02:13.5995326Z file:.git/config submodule.src/tools/rls.active=true
2019-12-30T12:02:13.5995489Z file:.git/config submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-12-30T12:02:13.5995741Z file:.git/config submodule.src/rust-installer.active=true
2019-12-30T12:02:13.5995936Z file:.git/config submodule.src/rust-installer.url=***-installer.git
2019-12-30T12:02:13.5996288Z file:.git/config submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-12-30T12:02:13.9072626Z      DOS    UNIX     MAC  BOM       TXTBIN  FILE
2019-12-30T12:02:13.9075116Z        0    5206       0  no_bom    text    Cargo.lock
2019-12-30T12:02:13.9075483Z        0     987       0  no_bom    text    src/tools/rust-installer/install-template.sh
---
2019-12-30T12:10:50.5669362Z    Compiling panic_unwind v0.0.0 (D:\a\1\s\src\libpanic_unwind)
2019-12-30T12:10:50.8176156Z error: unused variable: `dest`
2019-12-30T12:10:50.8176529Z    --> src\libpanic_unwind\seh.rs:222:52
2019-12-30T12:10:50.8176592Z     |
2019-12-30T12:10:50.8176643Z 222 |         unsafe extern "thiscall" fn exception_copy(dest: *mut [u64; 2],
2019-12-30T12:10:50.8176744Z     |                                                    ^^^^ help: consider prefixing with an underscore: `_dest`
2019-12-30T12:10:50.8176950Z     = note: `-D unused-variables` implied by `-D warnings`
2019-12-30T12:10:50.8176998Z 
2019-12-30T12:10:50.8177040Z error: unused variable: `src`
2019-12-30T12:10:50.8177083Z    --> src\libpanic_unwind\seh.rs:223:52
2019-12-30T12:10:50.8177083Z    --> src\libpanic_unwind\seh.rs:223:52
2019-12-30T12:10:50.8177169Z     |
2019-12-30T12:10:50.8177221Z 223 | ...                   src: *mut [u64; 2])
2019-12-30T12:10:50.8177277Z     |                       ^^^ help: consider prefixing with an underscore: `_src`
2019-12-30T12:10:50.8177395Z error: aborting due to 2 previous errors
2019-12-30T12:10:50.8177426Z 
2019-12-30T12:10:50.8258927Z error: could not compile `panic_unwind`.
2019-12-30T12:10:50.8259021Z warning: build failed, waiting for other jobs to finish...
2019-12-30T12:10:50.8259021Z warning: build failed, waiting for other jobs to finish...
2019-12-30T12:10:51.8145308Z error: build failed
2019-12-30T12:10:51.8193577Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\1\\s\\src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-30T12:10:51.8194051Z expected success, got: exit code: 101
2019-12-30T12:10:51.8237847Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2019-12-30T12:10:51.8238813Z Build completed unsuccessfully in 0:04:22
2019-12-30T12:10:51.8311361Z make: *** [Makefile:80: ci-subset-1] Error 1
2019-12-30T12:10:51.8479304Z   local time: Mon Dec 30 12:10:51 CUT 2019
2019-12-30T12:10:51.9247083Z   network time: Mon, 30 Dec 2019 12:10:51 GMT
2019-12-30T12:10:51.9278748Z == end clock drift check ==
2019-12-30T12:10:51.9304774Z 
2019-12-30T12:10:51.9304774Z 
2019-12-30T12:10:52.0542013Z ##[error]Bash exited with code '2'.
2019-12-30T12:10:52.0963578Z ##[section]Starting: Checkout
2019-12-30T12:10:52.1089118Z ==============================================================================
2019-12-30T12:10:52.1089209Z Task         : Get sources
2019-12-30T12:10:52.1089261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
