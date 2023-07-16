plain
2019-12-19T19:23:04.2883147Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-19T19:23:04.2883293Z 
2019-12-19T19:23:04.2883452Z   git checkout -b <new-branch-name>
2019-12-19T19:23:04.2883605Z 
2019-12-19T19:23:04.2883792Z HEAD is now at 1d27e9e09 Auto merge of #67435 - Mark-Simulacrum:rollup-tvlt9px, r=Mark-Simulacrum
2019-12-19T19:23:04.3251140Z ##[section]Starting: Setup environment
2019-12-19T19:23:04.3372488Z ==============================================================================
2019-12-19T19:23:04.3372730Z Task         : Bash
2019-12-19T19:23:04.3372821Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-19T19:23:19.6454150Z Chocolatey v0.10.15
2019-12-19T19:24:06.1245335Z Installing the following packages:
2019-12-19T19:24:06.1249655Z msys2
2019-12-19T19:24:06.1254994Z By installing you accept licenses for the packages.
2019-12-19T19:25:46.9708609Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-19T19:25:46.9709639Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-19T19:25:46.9717597Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-19T19:25:46.9718177Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-19T19:25:46.9718695Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-19T19:25:46.9719183Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-19T19:25:46.9720387Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-19T19:25:46.9720826Z  assistance.
2019-12-19T19:25:46.9831223Z 
2019-12-19T19:25:46.9831489Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-19T19:25:46.9831489Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-19T19:25:46.9832545Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-19T19:25:46.9837302Z 
2019-12-19T19:25:46.9841909Z Failures
2019-12-19T19:25:46.9849102Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-19T19:25:46.9849766Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-19T19:25:46.9850241Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-19T19:25:46.9850727Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-19T19:25:46.9851939Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-19T19:25:46.9852535Z  assistance.
2019-12-19T19:25:46.9862810Z 
2019-12-19T19:25:46.9863432Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-12-19T19:25:46.9863432Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-12-19T19:25:46.9863874Z experience to the next level at
2019-12-19T19:25:46.9864315Z  https://chocolatey.org/compare
2019-12-19T19:25:47.3419286Z 
2019-12-19T19:25:47.3523876Z ##[error]Bash exited with code '1'.
2019-12-19T19:25:47.3706235Z ##[section]Starting: Checkout
2019-12-19T19:25:47.3822034Z ==============================================================================
2019-12-19T19:25:47.3822196Z Task         : Get sources
2019-12-19T19:25:47.3822296Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
