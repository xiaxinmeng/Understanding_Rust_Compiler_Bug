plain
2019-12-23T00:24:12.6661056Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-23T00:24:12.6663602Z 
2019-12-23T00:24:12.6663870Z   git checkout -b <new-branch-name>
2019-12-23T00:24:12.6664076Z 
2019-12-23T00:24:12.6919233Z HEAD is now at 323c7ec85 Auto merge of #67540 - Mark-Simulacrum:fmt-the-world, r=Centril
2019-12-23T00:24:12.7056025Z ##[section]Starting: Setup environment
2019-12-23T00:24:12.7166631Z ==============================================================================
2019-12-23T00:24:12.7166750Z Task         : Bash
2019-12-23T00:24:12.7166832Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-23T00:24:27.4980949Z Chocolatey v0.10.15
2019-12-23T00:25:20.5568253Z Installing the following packages:
2019-12-23T00:25:20.5571460Z msys2
2019-12-23T00:25:20.5576862Z By installing you accept licenses for the packages.
2019-12-23T00:27:01.4198081Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-23T00:27:01.4198521Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-23T00:27:01.4206824Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:27:01.4206973Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:27:01.4207169Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:27:01.4207320Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:27:01.4207545Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:27:01.4207633Z  assistance.
2019-12-23T00:27:01.4336545Z 
2019-12-23T00:27:01.4336794Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:27:01.4336794Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:27:01.4337168Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-23T00:27:01.4342301Z 
2019-12-23T00:27:01.4347779Z Failures
2019-12-23T00:27:01.4355808Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:27:01.4355988Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:27:01.4356091Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:27:01.4356258Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:27:01.4356480Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:27:01.4356602Z  assistance.
2019-12-23T00:27:01.7907994Z 
2019-12-23T00:27:01.7907994Z 
2019-12-23T00:27:01.8003557Z ##[error]Bash exited with code '1'.
2019-12-23T00:27:01.8198870Z ##[section]Starting: Checkout
2019-12-23T00:27:01.8322144Z ==============================================================================
2019-12-23T00:27:01.8322296Z Task         : Get sources
2019-12-23T00:27:01.8322389Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
