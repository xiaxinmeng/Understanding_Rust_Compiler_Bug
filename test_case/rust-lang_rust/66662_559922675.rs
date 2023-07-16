plain
2019-11-30T07:52:09.6128341Z GeckoWebDriver=C:\SeleniumWebDrivers\GeckoDriver
2019-11-30T07:52:09.6128603Z HOME=/c/Users/VssAdministrator
2019-11-30T07:52:09.6128664Z HOMEDRIVE=C:
2019-11-30T07:52:09.6128736Z HOMEPATH=\Users\VssAdministrator
2019-11-30T07:52:09.6128850Z I also converted two tests from using `thread::spawn(...).join()` just for catching panics, to `catch_panic`, so that Miri can run them.
2019-11-30T07:52:09.6129026Z INPUT_ARGUMENTS=
2019-11-30T07:52:09.6129084Z ImageVersion=20191028.1
2019-11-30T07:52:09.6129176Z JAVA_HOME=C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64
2019-11-30T07:52:09.6129265Z JAVA_HOME_11_X64=C:\Program Files\Java\zulu-11-azure-jdk_11.33.15-11.0.4-win_x64
---
2019-11-30T07:53:52.2747426Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T07:53:52.2747599Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T07:53:52.2747719Z 
2019-11-30T07:53:52.2748019Z Failures
2019-11-30T07:53:52.2748157Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T07:53:52.2748328Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T07:53:52.2748596Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-11-30T07:53:52.2748743Z experience to the next level at
2019-11-30T07:53:52.2748975Z  https://chocolatey.org/compare
2019-11-30T07:53:52.4386880Z 
2019-11-30T07:53:52.4386880Z 
2019-11-30T07:53:52.4457631Z ##[error]Bash exited with code '1'.
2019-11-30T07:53:52.4584304Z ##[section]Starting: Checkout
2019-11-30T07:53:52.4671848Z ==============================================================================
2019-11-30T07:53:52.4671935Z Task         : Get sources
2019-11-30T07:53:52.4672011Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
