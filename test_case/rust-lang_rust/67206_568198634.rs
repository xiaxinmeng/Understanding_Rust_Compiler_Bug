plain
2019-12-21T17:36:46.6347097Z Chocolatey v0.10.15
2019-12-21T17:37:35.3128895Z Installing the following packages:
2019-12-21T17:37:35.3133027Z msys2
2019-12-21T17:37:35.3137839Z By installing you accept licenses for the packages.
2019-12-21T17:39:16.0564457Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-21T17:39:16.0565204Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-21T17:39:16.0572852Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:39:16.0573321Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:39:16.0573704Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:39:16.0574072Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:39:16.0574923Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:39:16.0575434Z  assistance.
2019-12-21T17:39:16.0674427Z 
2019-12-21T17:39:16.0675289Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:39:16.0675289Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:39:16.0676008Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-21T17:39:16.0679753Z 
2019-12-21T17:39:16.0685231Z Failures
2019-12-21T17:39:16.0692518Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:39:16.0693154Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:39:16.0693554Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:39:16.0693947Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:39:16.0695029Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:39:16.0695387Z  assistance.
2019-12-21T17:39:16.4170072Z 
2019-12-21T17:39:16.4170072Z 
2019-12-21T17:39:16.4243717Z ##[error]Bash exited with code '1'.
2019-12-21T17:39:16.4349416Z ##[section]Starting: Checkout
2019-12-21T17:39:16.4447913Z ==============================================================================
2019-12-21T17:39:16.4447996Z Task         : Get sources
2019-12-21T17:39:16.4448095Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
