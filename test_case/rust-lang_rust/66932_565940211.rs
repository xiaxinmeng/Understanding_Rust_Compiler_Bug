plain
2019-12-16T07:32:08.0319186Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-16T07:32:08.0319288Z 
2019-12-16T07:32:08.0319353Z   git checkout -b <new-branch-name>
2019-12-16T07:32:08.0319419Z 
2019-12-16T07:32:08.0319501Z HEAD is now at e7cbc2bb0 Auto merge of #66932 - rust-lang:pass-check-runfail, r=petrochenkov
2019-12-16T07:32:08.0654194Z ##[section]Starting: Setup environment
2019-12-16T07:32:08.0758883Z ==============================================================================
2019-12-16T07:32:08.0759411Z Task         : Bash
2019-12-16T07:32:08.0759507Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-16T07:32:09.7318217Z BUILD_SOURCEBRANCHNAME=auto
2019-12-16T07:32:09.7318304Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-16T07:32:09.7318388Z BUILD_SOURCEVERSION=e7cbc2bb01f9e4a95e3048b94116609b0f80e612
2019-12-16T07:32:09.7318482Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-16T07:32:09.7318615Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66932 - rust-lang:pass-check-runfail, r=petrochenkov
2019-12-16T07:32:09.7318788Z CI_JOB_NAME=i686-msvc-1
2019-12-16T07:32:09.7318857Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-16T07:32:09.7318957Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-16T07:32:09.7319039Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-16T07:32:09.7335144Z ProgramFiles(x86)=C:\Program Files (x86)
2019-12-16T07:32:09.7335251Z ProgramW6432=C:\Program Files
2019-12-16T07:32:09.7335349Z RUNNER_TOOLSDIRECTORY=C:/hostedtoolcache/windows
2019-12-16T07:32:09.7335448Z RUST_CONFIGURE_ARGS=--build=i686-pc-windows-msvc
2019-12-16T07:32:09.7335655Z Revamp `// run-fail` wrt. `--pass` & support `// build-fail` & `// check-fail`
2019-12-16T07:32:09.7336025Z Revamp how `// run-fail` tests work internally by having a separate `FailMode` that does not interfere with `PassMode`. In particular, `--pass check` will now have no effect on `// *-fail` tests. Moreover, new test annotations `// check-fail` (the default) and `// build-fail` are added. The latter is useful to distinguish post-monomorphization failures from pre-monomorphization failures as seen throughout the PR. Finally, ensure that non-`Ui` tests do not listen to `--pass check` such that the flag can be used with e.g. `./x.py test --pass check` directly.
2019-12-16T07:32:09.7336349Z SCCACHE_AWS_ACCESS_KEY_ID=AKIA46X5W6CZPECECL6V
2019-12-16T07:32:09.7336444Z SCCACHE_BUCKET=rust-lang-ci-sccache2
2019-12-16T07:32:09.7336515Z SCRIPT=make ci-subset-1
2019-12-16T07:32:09.7336598Z SHLVL=2
---
2019-12-16T07:33:51.3768189Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-16T07:33:51.3768313Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-16T07:33:51.3772290Z 
2019-12-16T07:33:51.3776176Z Failures
2019-12-16T07:33:51.3782293Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-16T07:33:51.3782598Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-16T07:33:51.8377762Z 
2019-12-16T07:33:51.8481655Z ##[error]Bash exited with code '1'.
2019-12-16T07:33:51.8635273Z ##[section]Starting: Checkout
2019-12-16T07:33:51.8789479Z ==============================================================================
2019-12-16T07:33:51.8789593Z Task         : Get sources
2019-12-16T07:33:51.8789683Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
