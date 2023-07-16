plain
2019-12-21T17:48:15.3436808Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-21T17:48:15.3436939Z 
2019-12-21T17:48:15.3437100Z   git checkout -b <new-branch-name>
2019-12-21T17:48:15.3437403Z 
2019-12-21T17:48:15.3437745Z HEAD is now at 102b7cd8e Auto merge of #67493 - Centril:rollup-m8eeyh3, r=Centril
2019-12-21T17:48:15.3780573Z ##[section]Starting: Setup environment
2019-12-21T17:48:15.3894540Z ==============================================================================
2019-12-21T17:48:15.3894694Z Task         : Bash
2019-12-21T17:48:15.3894797Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-21T17:48:30.4211732Z Chocolatey v0.10.15
2019-12-21T17:49:22.0117449Z Installing the following packages:
2019-12-21T17:49:22.0122047Z msys2
2019-12-21T17:49:22.0127243Z By installing you accept licenses for the packages.
2019-12-21T17:51:02.8099893Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-21T17:51:02.8100919Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-21T17:51:02.8111692Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:51:02.8112326Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:51:02.8112783Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:51:02.8113144Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:51:02.8113638Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:51:02.8114203Z  assistance.
2019-12-21T17:51:02.8257301Z 
2019-12-21T17:51:02.8257965Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:51:02.8257965Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:51:02.8258284Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-21T17:51:02.8264409Z 
2019-12-21T17:51:02.8270487Z Failures
2019-12-21T17:51:02.8283895Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:51:02.8284484Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:51:02.8284745Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:51:02.8285052Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:51:02.8285536Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:51:02.8285741Z  assistance.
2019-12-21T17:51:03.1847145Z 
2019-12-21T17:51:03.1847145Z 
2019-12-21T17:51:03.1950011Z ##[error]Bash exited with code '1'.
2019-12-21T17:51:03.2096244Z ##[section]Starting: Checkout
2019-12-21T17:51:03.2213950Z ==============================================================================
2019-12-21T17:51:03.2214059Z Task         : Get sources
2019-12-21T17:51:03.2214135Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
