plain
2019-12-15T04:56:03.3990119Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-15T04:56:03.3990248Z 
2019-12-15T04:56:03.3990402Z   git checkout -b <new-branch-name>
2019-12-15T04:56:03.3990708Z 
2019-12-15T04:56:03.3990972Z HEAD is now at d3a6c0f96 Auto merge of #66932 - rust-lang:pass-check-runfail, r=petrochenkov
2019-12-15T04:56:03.4310819Z ##[section]Starting: Setup environment
2019-12-15T04:56:03.4430765Z ==============================================================================
2019-12-15T04:56:03.4430859Z Task         : Bash
2019-12-15T04:56:03.4430945Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-15T04:56:04.9851317Z BUILD_SOURCEBRANCHNAME=auto
2019-12-15T04:56:04.9851394Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-15T04:56:04.9851470Z BUILD_SOURCEVERSION=d3a6c0f96acbad1d52eb9e9ed19d9cc16a55d66b
2019-12-15T04:56:04.9851553Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-15T04:56:04.9851644Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66932 - rust-lang:pass-check-runfail, r=petrochenkov
2019-12-15T04:56:04.9851798Z CI_JOB_NAME=x86_64-mingw-2
2019-12-15T04:56:04.9851873Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-15T04:56:04.9851958Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-15T04:56:04.9852037Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-15T04:56:04.9860377Z ProgramFiles(x86)=C:\Program Files (x86)
2019-12-15T04:56:04.9860453Z ProgramW6432=C:\Program Files
2019-12-15T04:56:04.9860522Z RUNNER_TOOLSDIRECTORY=C:/hostedtoolcache/windows
2019-12-15T04:56:04.9860608Z RUST_CONFIGURE_ARGS=--build=x86_64-pc-windows-gnu
2019-12-15T04:56:04.9860702Z Revamp `// run-fail` wrt. `--pass` & support `// build-fail` & `// check-fail`
2019-12-15T04:56:04.9860999Z Revamp how `// run-fail` tests work internally by having a separate `FailMode` that does not interfere with `PassMode`. In particular, `--pass check` will now have no effect on `// *-fail` tests. Moreover, new test annotations `// check-fail` (the default) and `// build-fail` are added. The latter is useful to distinguish post-monomorphization failures from pre-monomorphization failures as seen throughout the PR. Finally, ensure that non-`Ui` tests do not listen to `--pass check` such that the flag can be used with e.g. `./x.py test --pass check` directly.
2019-12-15T04:56:04.9861337Z SCCACHE_AWS_ACCESS_KEY_ID=AKIA46X5W6CZPECECL6V
2019-12-15T04:56:04.9861403Z SCCACHE_BUCKET=rust-lang-ci-sccache2
2019-12-15T04:56:04.9861479Z SCRIPT=make ci-mingw-subset-2
2019-12-15T04:56:04.9861553Z SHLVL=2
---
2019-12-15T04:56:17.1680078Z Chocolatey v0.10.15
2019-12-15T04:57:07.6367648Z Installing the following packages:
2019-12-15T04:57:07.6371046Z msys2
2019-12-15T04:57:07.6375836Z By installing you accept licenses for the packages.
2019-12-15T04:58:48.4672523Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-15T04:58:48.4673551Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-15T04:58:48.4680924Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-15T04:58:48.4681449Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-15T04:58:48.4681721Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-15T04:58:48.4682000Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-15T04:58:48.4682427Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-15T04:58:48.4682674Z  assistance.
2019-12-15T04:58:48.4774407Z 
2019-12-15T04:58:48.4774932Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T04:58:48.4774932Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T04:58:48.4775160Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-15T04:58:48.4779263Z 
2019-12-15T04:58:48.4782793Z Failures
2019-12-15T04:58:48.4788647Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-15T04:58:48.4789075Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-15T04:58:48.4789268Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-15T04:58:48.4789553Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-15T04:58:48.4789978Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-15T04:58:48.4790193Z  assistance.
2019-12-15T04:58:48.8536345Z 
2019-12-15T04:58:48.8536345Z 
2019-12-15T04:58:48.8621479Z ##[error]Bash exited with code '1'.
2019-12-15T04:58:48.8737625Z ##[section]Starting: Checkout
2019-12-15T04:58:48.8845235Z ==============================================================================
2019-12-15T04:58:48.8845374Z Task         : Get sources
2019-12-15T04:58:48.8845463Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
