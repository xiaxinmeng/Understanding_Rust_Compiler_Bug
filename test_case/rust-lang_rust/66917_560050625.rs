plain
2019-12-01T03:54:33.7621519Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T03:54:33.7622016Z 
2019-12-01T03:54:33.7622334Z   git checkout -b <new-branch-name>
2019-12-01T03:54:33.7622504Z 
2019-12-01T03:54:33.7622696Z HEAD is now at 8398ce625 Auto merge of #66917 - Centril:rollup-xj2enik, r=Centril
2019-12-01T03:54:33.8018751Z ##[section]Starting: Setup environment
2019-12-01T03:54:33.8127680Z ==============================================================================
2019-12-01T03:54:33.8127789Z Task         : Bash
2019-12-01T03:54:33.8127892Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T03:54:35.6156744Z 
2019-12-01T03:54:35.6156785Z 
2019-12-01T03:54:35.6156823Z 
2019-12-01T03:54:35.6156875Z 
2019-12-01T03:54:35.6157001Z  - #66503 (More useful test error messages on should_panic(expected=...) mismatch)
2019-12-01T03:54:35.6157159Z  - #66662 (Miri: run panic-catching tests in liballoc)
2019-12-01T03:54:35.6157267Z  - #66679 (Improve lifetime errors with implicit trait object lifetimes)
2019-12-01T03:54:35.6157389Z  - #66726 (Use recursion_limit for const eval stack limit)
2019-12-01T03:54:35.6157495Z  - #66790 (Do `min_const_fn` checks for `SetDiscriminant`s target)
2019-12-01T03:54:35.6157639Z  - #66832 (const_prop: detect and avoid catching Miri errors that require allocation)
2019-12-01T03:54:35.6157862Z  - #66890 (Format liballoc with rustfmt)
2019-12-01T03:54:35.6157970Z  - #66896 (pass Queries to compiler callbacks)
2019-12-01T03:54:35.6158054Z AGENT_BUILDDIRECTORY=D:\a\1
2019-12-01T03:54:35.6158166Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
---
2019-12-01T03:54:35.6167319Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T03:54:35.6167413Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T03:54:35.6167508Z BUILD_SOURCEVERSION=8398ce625ef260401b272dac0f9913b8fdbe99a4
2019-12-01T03:54:35.6167611Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T03:54:35.6167714Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66917 - Centril:rollup-xj2enik, r=Centril
2019-12-01T03:54:35.6167930Z CI_JOB_NAME=dist-i686-mingw
2019-12-01T03:54:35.6168010Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T03:54:35.6168179Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T03:54:35.6168298Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T03:54:51.7310010Z Chocolatey v0.10.15
2019-12-01T03:55:34.6651638Z Installing the following packages:
2019-12-01T03:55:34.6658246Z msys2
2019-12-01T03:55:34.6664344Z By installing you accept licenses for the packages.
2019-12-01T03:57:15.5118382Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-01T03:57:15.5119307Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-01T03:57:15.5127204Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T03:57:15.5127832Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T03:57:15.5128303Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T03:57:15.5128535Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T03:57:15.5129194Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T03:57:15.5129388Z  assistance.
2019-12-01T03:57:15.5276738Z 
2019-12-01T03:57:15.5277395Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T03:57:15.5277395Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T03:57:15.5278360Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T03:57:15.5282739Z 
2019-12-01T03:57:15.5288069Z Failures
2019-12-01T03:57:15.5295160Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T03:57:15.5295781Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T03:57:15.5296285Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T03:57:15.5296770Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T03:57:15.5297684Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T03:57:15.5298156Z  assistance.
2019-12-01T03:57:15.8880869Z 
2019-12-01T03:57:15.8880869Z 
2019-12-01T03:57:15.8965207Z ##[error]Bash exited with code '1'.
2019-12-01T03:57:15.9111033Z ##[section]Starting: Checkout
2019-12-01T03:57:15.9235414Z ==============================================================================
2019-12-01T03:57:15.9235535Z Task         : Get sources
2019-12-01T03:57:15.9235624Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
