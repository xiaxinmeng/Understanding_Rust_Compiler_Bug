plain
2019-12-15T05:22:38.9578725Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-15T05:22:38.9579728Z 
2019-12-15T05:22:38.9586441Z   git checkout -b <new-branch-name>
2019-12-15T05:22:38.9586881Z 
2019-12-15T05:22:38.9591532Z HEAD is now at 83044ce9a Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T05:22:38.9963081Z ##[section]Starting: Setup environment
2019-12-15T05:22:39.0062773Z ==============================================================================
2019-12-15T05:22:39.0062869Z Task         : Bash
2019-12-15T05:22:39.0062933Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-15T05:22:40.5921216Z BUILD_SOURCEBRANCHNAME=auto
2019-12-15T05:22:40.5921277Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-15T05:22:40.5921366Z BUILD_SOURCEVERSION=83044ce9aa535c9ff051ba5092f2e11d32a58dc7
2019-12-15T05:22:40.5921440Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-15T05:22:40.5921535Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T05:22:40.5921699Z CI_JOB_NAME=x86_64-mingw-1
2019-12-15T05:22:40.5921776Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-15T05:22:40.5921845Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-15T05:22:40.5921938Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-15T05:22:52.7661338Z Chocolatey v0.10.15
2019-12-15T05:23:39.9608459Z Installing the following packages:
2019-12-15T05:23:39.9612215Z msys2
2019-12-15T05:23:39.9616465Z By installing you accept licenses for the packages.
2019-12-15T05:25:20.7226503Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-15T05:25:21.6868374Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-15T05:25:21.6870036Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-15T05:25:21.6870275Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-15T05:25:21.6870482Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-15T05:25:21.6870708Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-15T05:25:21.6871070Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-15T05:25:21.6871261Z  assistance.
2019-12-15T05:25:21.6871386Z 
2019-12-15T05:25:21.6871580Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T05:25:21.6871580Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T05:25:21.6871760Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-15T05:25:21.6871925Z 
2019-12-15T05:25:21.6872072Z Failures
2019-12-15T05:25:21.6872276Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-15T05:25:21.6872461Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-15T05:25:21.6872690Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-15T05:25:21.6872902Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-15T05:25:21.6873270Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-15T05:25:21.6873454Z  assistance.
2019-12-15T05:25:21.6873577Z 
2019-12-15T05:25:21.6873577Z 
2019-12-15T05:25:21.6928229Z ##[error]Bash exited with code '1'.
2019-12-15T05:25:21.7046552Z ##[section]Starting: Checkout
2019-12-15T05:25:21.7157976Z ==============================================================================
2019-12-15T05:25:21.7158084Z Task         : Get sources
2019-12-15T05:25:21.7158164Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
