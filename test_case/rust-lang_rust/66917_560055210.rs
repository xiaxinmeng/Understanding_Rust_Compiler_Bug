plain
2019-12-01T04:44:20.8529849Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T04:44:20.8530080Z 
2019-12-01T04:44:20.8530153Z   git checkout -b <new-branch-name>
2019-12-01T04:44:20.8530268Z 
2019-12-01T04:44:20.8530748Z HEAD is now at c50366f77 Auto merge of #66917 - Centril:rollup-xj2enik, r=Centril
2019-12-01T04:44:20.8864628Z ##[section]Starting: Setup environment
2019-12-01T04:44:20.8978288Z ==============================================================================
2019-12-01T04:44:20.8978401Z Task         : Bash
2019-12-01T04:44:20.8978479Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T04:44:22.6137451Z 
2019-12-01T04:44:22.6137604Z 
2019-12-01T04:44:22.6137734Z 
2019-12-01T04:44:22.6137896Z 
2019-12-01T04:44:22.6138112Z  - #66503 (More useful test error messages on should_panic(expected=...) mismatch)
2019-12-01T04:44:22.6138333Z  - #66662 (Miri: run panic-catching tests in liballoc)
2019-12-01T04:44:22.6138544Z  - #66679 (Improve lifetime errors with implicit trait object lifetimes)
2019-12-01T04:44:22.6138772Z  - #66726 (Use recursion_limit for const eval stack limit)
2019-12-01T04:44:22.6138974Z  - #66790 (Do `min_const_fn` checks for `SetDiscriminant`s target)
2019-12-01T04:44:22.6139188Z  - #66832 (const_prop: detect and avoid catching Miri errors that require allocation)
2019-12-01T04:44:22.6139600Z  - #66890 (Format liballoc with rustfmt)
2019-12-01T04:44:22.6139780Z  - #66896 (pass Queries to compiler callbacks)
2019-12-01T04:44:22.6139969Z AGENT_BUILDDIRECTORY=D:\a\1
2019-12-01T04:44:22.6140159Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
---
2019-12-01T04:44:22.6152305Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T04:44:22.6152483Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T04:44:22.6152689Z BUILD_SOURCEVERSION=c50366f77373c8a41902f604025cb3b5c349fad2
2019-12-01T04:44:22.6152863Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T04:44:22.6153067Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66917 - Centril:rollup-xj2enik, r=Centril
2019-12-01T04:44:22.6153441Z CI_JOB_NAME=x86_64-mingw-1
2019-12-01T04:44:22.6153637Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T04:44:22.6153810Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T04:44:22.6154004Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T04:44:38.1139765Z Chocolatey v0.10.15
2019-12-01T04:45:23.4800107Z Installing the following packages:
2019-12-01T04:45:23.4804452Z msys2
2019-12-01T04:45:23.4809076Z By installing you accept licenses for the packages.
2019-12-01T04:47:04.3291102Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-01T04:47:04.3326289Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-01T04:47:04.3361872Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T04:47:04.3416945Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T04:47:04.3417378Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T04:47:04.3417638Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T04:47:04.3418093Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T04:47:04.3418318Z  assistance.
2019-12-01T04:47:04.3418474Z 
2019-12-01T04:47:04.3418694Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T04:47:04.3418694Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T04:47:04.3418932Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T04:47:04.3419114Z 
2019-12-01T04:47:04.3419309Z Failures
2019-12-01T04:47:04.3419512Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T04:47:04.3419749Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T04:47:04.3419956Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T04:47:04.3420202Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T04:47:04.3420633Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T04:47:04.3420849Z  assistance.
2019-12-01T04:47:04.3420992Z 
2019-12-01T04:47:04.3421202Z Did you know the proceeds of Pro (and some proceeds from other
2019-12-01T04:47:04.3421202Z Did you know the proceeds of Pro (and some proceeds from other
2019-12-01T04:47:04.3421544Z  licensed editions) go into bettering the community infrastructure?
2019-12-01T04:47:04.3421807Z  Your support ensures an active community, keeps Chocolatey tip top,
2019-12-01T04:47:04.3422043Z  plus it nets you some awesome features!
2019-12-01T04:47:04.3422228Z  https://chocolatey.org/compare
2019-12-01T04:47:04.7014699Z 
2019-12-01T04:47:04.7082826Z ##[error]Bash exited with code '1'.
2019-12-01T04:47:04.7215155Z ##[section]Starting: Checkout
2019-12-01T04:47:04.7339627Z ==============================================================================
2019-12-01T04:47:04.7339757Z Task         : Get sources
2019-12-01T04:47:04.7339860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
