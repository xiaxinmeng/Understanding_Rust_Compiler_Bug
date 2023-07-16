plain
2019-12-23T00:08:09.4720747Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-23T00:08:09.4720980Z 
2019-12-23T00:08:09.4721201Z   git checkout -b <new-branch-name>
2019-12-23T00:08:09.4721371Z 
2019-12-23T00:08:09.4721568Z HEAD is now at 3297c06dd Auto merge of #67540 - Mark-Simulacrum:fmt-the-world, r=Centril
2019-12-23T00:08:09.5109490Z ##[section]Starting: Setup environment
2019-12-23T00:08:09.5231355Z ==============================================================================
2019-12-23T00:08:09.5231471Z Task         : Bash
2019-12-23T00:08:09.5231549Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-23T00:08:26.2826207Z Chocolatey v0.10.15
2019-12-23T00:09:23.4637312Z Installing the following packages:
2019-12-23T00:09:23.4642673Z msys2
2019-12-23T00:09:23.4647204Z By installing you accept licenses for the packages.
2019-12-23T00:11:04.4445875Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-23T00:11:04.4446620Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-23T00:11:04.4454507Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:11:04.4454805Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:11:04.4455082Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:11:04.4455288Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:11:04.4455972Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:11:04.4456165Z  assistance.
2019-12-23T00:11:04.4565790Z 
2019-12-23T00:11:04.4566457Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:11:04.4566457Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:11:04.4566841Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-23T00:11:04.4570882Z 
2019-12-23T00:11:04.4576917Z Failures
2019-12-23T00:11:04.4585630Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:11:04.4586120Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:11:04.4586497Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:11:04.4586944Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:11:04.4587797Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:11:04.4588702Z  assistance.
2019-12-23T00:11:04.8185375Z 
2019-12-23T00:11:04.8185375Z 
2019-12-23T00:11:04.8276328Z ##[error]Bash exited with code '1'.
2019-12-23T00:11:04.8407545Z ##[section]Starting: Checkout
2019-12-23T00:11:04.8567515Z ==============================================================================
2019-12-23T00:11:04.8567599Z Task         : Get sources
2019-12-23T00:11:04.8567711Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
