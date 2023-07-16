plain
2019-11-30T17:27:32.7457016Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T17:27:32.7457065Z 
2019-11-30T17:27:32.7457560Z   git checkout -b <new-branch-name>
2019-11-30T17:27:32.7457638Z 
2019-11-30T17:27:32.7457730Z HEAD is now at a2ab543ca Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T17:27:32.7821268Z ##[section]Starting: Setup environment
2019-11-30T17:27:32.7929666Z ==============================================================================
2019-11-30T17:27:32.7929951Z Task         : Bash
2019-11-30T17:27:32.7930204Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T17:27:34.5222614Z 
2019-11-30T17:27:34.5222706Z 
2019-11-30T17:27:34.5222740Z 
2019-11-30T17:27:34.5222847Z  - #66612 (Initial implementation of or-pattern usefulness checking)
2019-11-30T17:27:34.5222951Z  - #66705 (Atomic as_mut_ptr)
2019-11-30T17:27:34.5223542Z  - #66759 (impl TrustedLen for vec::Drain)
2019-11-30T17:27:34.5223655Z  - #66858 (Use LLVMAddAnalysisPasses instead of Rust's wrapper)
2019-11-30T17:27:34.5223775Z  - #66870 (SimplifyArmIdentity only for locals with the same type)
2019-11-30T17:27:34.5223879Z  - #66883 (rustc_typeck: gate AnonConst's generics on feature(const_generics).)
2019-11-30T17:27:34.5224031Z  - #66889 (Make python-generated source files compatible with rustfmt)
2019-11-30T17:27:34.5224137Z  - #66894 (Remove unneeded prelude imports in libcore tests)
2019-11-30T17:27:34.5224269Z  - #66895 (Feature gating *declarations* => new crate `rustc_feature`)
2019-11-30T17:27:34.5224593Z AGENT_BUILDDIRECTORY=D:\a\1
2019-11-30T17:27:34.5224704Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T17:27:34.5224793Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T17:27:34.5225730Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
---
2019-11-30T17:27:34.5233298Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T17:27:34.5233385Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T17:27:34.5234027Z BUILD_SOURCEVERSION=a2ab543ca7a5767b309a4f441ea6e81488786ede
2019-11-30T17:27:34.5236318Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T17:27:34.5236482Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T17:27:34.5236667Z CI_JOB_NAME=dist-i686-mingw
2019-11-30T17:27:34.5236768Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T17:27:34.5236870Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T17:27:34.5236958Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T17:27:49.9953812Z Chocolatey v0.10.15
2019-11-30T17:28:42.7005174Z Installing the following packages:
2019-11-30T17:28:42.7008900Z msys2
2019-11-30T17:28:42.7015168Z By installing you accept licenses for the packages.
2019-11-30T17:30:23.5903449Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-11-30T17:30:23.5904347Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-11-30T17:30:23.5911859Z msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T17:30:23.5912415Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T17:30:23.5912893Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T17:30:23.5913157Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T17:30:23.5913577Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T17:30:23.5913787Z  assistance.
2019-11-30T17:30:23.6025095Z 
2019-11-30T17:30:23.6025776Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T17:30:23.6025776Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T17:30:23.6026229Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T17:30:23.6030502Z 
2019-11-30T17:30:23.6034899Z Failures
2019-11-30T17:30:23.6044107Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T17:30:23.6044660Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T17:30:23.6044910Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T17:30:23.6045142Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T17:30:23.6045572Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T17:30:23.6045784Z  assistance.
2019-11-30T17:30:23.9639104Z 
2019-11-30T17:30:23.9639104Z 
2019-11-30T17:30:23.9730629Z ##[error]Bash exited with code '1'.
2019-11-30T17:30:23.9913266Z ##[section]Starting: Checkout
2019-11-30T17:30:24.0031740Z ==============================================================================
2019-11-30T17:30:24.0031857Z Task         : Get sources
2019-11-30T17:30:24.0031944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
