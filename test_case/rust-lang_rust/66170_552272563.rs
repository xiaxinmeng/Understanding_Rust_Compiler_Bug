plain
2019-11-11T00:57:13.3054192Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-11T00:57:13.3054310Z 
2019-11-11T00:57:13.3054454Z   git checkout -b <new-branch-name>
2019-11-11T00:57:13.3054561Z 
2019-11-11T00:57:13.3054729Z HEAD is now at 47904d203 Auto merge of #66170 - ecstatic-morse:hir-const-check, r=Centril,oli-obk
2019-11-11T00:57:13.3388839Z ##[section]Starting: Decide whether to run this job
2019-11-11T00:57:13.3481730Z ==============================================================================
2019-11-11T00:57:13.3481845Z Task         : Bash
2019-11-11T00:57:13.3481910Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-11T00:57:14.5752819Z ANT_HOME=C:\ProgramData\chocolatey\lib\ant\apache-ant-1.10.5
2019-11-11T00:57:14.5752916Z APPDATA=C:\Users\VssAdministrator\AppData\Roaming
2019-11-11T00:57:14.5752993Z AZURE_EXTENSION_DIR=C:\Program Files\Common Files\AzureCliExtensionDirectory
2019-11-11T00:57:14.5753095Z AZURE_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-11T00:57:14.5753194Z Add a HIR pass to check consts for `if`, `loop`, etc.
2019-11-11T00:57:14.5753567Z BOOST_ROOT_1_69_0=C:\Program Files\Boost\1.69.0
2019-11-11T00:57:14.5753629Z BUILD_ARTIFACTSTAGINGDIRECTORY=D:\a\1\a
2019-11-11T00:57:14.5753704Z BUILD_BINARIESDIRECTORY=D:\a\1\b
2019-11-11T00:57:14.5753761Z BUILD_BUILDID=13213
---
2019-11-11T00:57:14.5756590Z BUILD_SOURCEBRANCHNAME=auto
2019-11-11T00:57:14.5756665Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-11T00:57:14.5756760Z BUILD_SOURCEVERSION=47904d2033588f5e713043fa59a214a41f158675
2019-11-11T00:57:14.5756826Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-11T00:57:14.5756949Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66170 - ecstatic-morse:hir-const-check, r=Centril,oli-obk
2019-11-11T00:57:14.5757115Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-11T00:57:14.5757188Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-11T00:57:14.5757273Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-11T00:57:14.5757347Z COMPUTERNAME=fv-az363
---
2019-11-11T00:57:14.5759022Z HOMEPATH=\Users\VssAdministrator
2019-11-11T00:57:14.5759089Z IEWebDriver=C:\SeleniumWebDrivers\IEDriver
2019-11-11T00:57:14.5759162Z INPUT_ARGUMENTS=
2019-11-11T00:57:14.5759214Z ImageVersion=20191028.1
2019-11-11T00:57:14.5759418Z In this implementation, the HIR const-checking pass is run much earlier than the MIR one, so it will supersede any errors from the latter. I will need some mentoring if we wish to change this, since I'm not familiar with the diagnostics system. Moving this pass into the same phase as the MIR const-checker could also help keep backwards compatibility for items like `const _: () = loop { break; };`, which are currently (erroneously?) accepted by the MIR const-checker (see #62272).
2019-11-11T00:57:14.5759824Z JAVA_HOME_11_X64=C:\Program Files\Java\zulu-11-azure-jdk_11.33.15-11.0.4-win_x64
2019-11-11T00:57:14.5759913Z JAVA_HOME_7_X64=C:\Program Files\Java\zulu-7-azure-jdk_7.31.0.5-7.0.232-win_x64
2019-11-11T00:57:14.5760480Z JAVA_HOME_8_X64=C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64
2019-11-11T00:57:14.5760595Z LOCALAPPDATA=C:\Users\VssAdministrator\AppData\Local
---
2019-11-11T00:57:14.5798778Z TMP=/tmp
2019-11-11T00:57:14.5798844Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-11T00:57:14.5799038Z TOOLSTATE_PUBLISH=1
2019-11-11T00:57:14.5799130Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-11T00:57:14.5799399Z This PR adds a HIR pass to check for high-level control flow constructs that are forbidden in a const-context. The MIR const-checker is unable to provide good spans for these since they are lowered to control flow primitives (e.g., `Goto` and `SwitchInt`), and these often don't map back to the underlying statement as a whole. This PR is intended only to improve diagnostics once `if` and `match` become commonplace in constants (behind a feature flag). The MIR const-checker will continue to operate unchanged, and will catch anything this check might miss.
2019-11-11T00:57:14.5799710Z USERDOMAIN=fv-az363
2019-11-11T00:57:14.5799808Z USERDOMAIN_ROAMINGPROFILE=fv-az363
2019-11-11T00:57:14.5799893Z USERNAME=VssAdministrator
2019-11-11T00:57:14.5799951Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-11T00:57:14.5800415Z WINDIR=C:\windows
2019-11-11T00:57:14.5800490Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-11T00:57:14.5800564Z _=/usr/bin/printenv
2019-11-11T00:57:14.5800619Z agent.jobstatus=Succeeded
2019-11-11T00:57:14.5800691Z cc @eddyb (since they filed #62272)
2019-11-11T00:57:14.5800829Z 
2019-11-11T00:57:14.5800880Z disk usage:
2019-11-11T00:57:14.6391075Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-11T00:57:14.6391204Z C:/Program Files/Git  256G  140G  116G  55% /
---
2019-11-11T02:50:17.2881527Z    Compiling hex v0.4.0
2019-11-11T02:50:20.0933443Z [RUSTC-TIMING] hex test:false 3.368
2019-11-11T02:50:20.0979591Z    Compiling shell-escape v0.1.4
2019-11-11T02:50:20.9053512Z [RUSTC-TIMING] shell_escape test:false 0.796
2019-11-11T02:50:20.9155500Z memory allocation of 2147483656 bytes failed[RUSTC-TIMING] hex test:false 3.619
2019-11-11T02:50:20.9199953Z error: could not compile `hex`.
2019-11-11T02:50:20.9200256Z warning: build failed, waiting for other jobs to finish...
2019-11-11T02:50:22.5108881Z [RUSTC-TIMING] glob test:false 1.585
2019-11-11T02:50:22.5510567Z error: build failed
---
2019-11-11T02:50:22.9158871Z   local time: Mon Nov 11 02:50:22 CUT 2019
2019-11-11T02:50:23.2659623Z   network time: Mon, 11 Nov 2019 02:50:23 GMT
2019-11-11T02:50:23.2670773Z == end clock drift check ==
2019-11-11T02:50:23.4138435Z 
2019-11-11T02:50:23.8255803Z ##[error]Bash exited with code '1'.
2019-11-11T02:50:23.8854297Z ##[section]Starting: Checkout
2019-11-11T02:50:23.9805773Z ==============================================================================
2019-11-11T02:50:23.9805993Z Task         : Get sources
2019-11-11T02:50:23.9806136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
