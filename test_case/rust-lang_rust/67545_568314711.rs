plain
2019-12-23T00:00:06.5809814Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-23T00:00:06.5810247Z 
2019-12-23T00:00:06.5810736Z   git checkout -b <new-branch-name>
2019-12-23T00:00:06.5810992Z 
2019-12-23T00:00:06.5811197Z HEAD is now at b992cc697 Auto merge of #67545 - Centril:rollup-t0n3lqk, r=Centril
2019-12-23T00:00:06.6112184Z ##[section]Starting: Setup environment
2019-12-23T00:00:06.6195205Z ==============================================================================
2019-12-23T00:00:06.6195279Z Task         : Bash
2019-12-23T00:00:06.6195353Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-23T00:00:22.2257215Z Chocolatey v0.10.15
2019-12-23T00:01:08.5891512Z Installing the following packages:
2019-12-23T00:01:08.5895113Z msys2
2019-12-23T00:01:08.5899394Z By installing you accept licenses for the packages.
2019-12-23T00:02:49.3491284Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-23T00:02:49.3491633Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-23T00:02:49.3501068Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:02:49.3501179Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:02:49.3501562Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:02:49.3501648Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:02:49.3501818Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:02:49.3502396Z  assistance.
2019-12-23T00:02:49.3617652Z 
2019-12-23T00:02:49.3617835Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:02:49.3617835Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-23T00:02:49.3617911Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-23T00:02:49.3621698Z 
2019-12-23T00:02:49.3625805Z Failures
2019-12-23T00:02:49.3632315Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-23T00:02:49.3632515Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-23T00:02:49.3632601Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-23T00:02:49.3632721Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-23T00:02:49.3632880Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-23T00:02:49.3632941Z  assistance.
2019-12-23T00:02:50.3429348Z 
2019-12-23T00:02:50.3429348Z 
2019-12-23T00:02:50.3478526Z ##[error]Bash exited with code '1'.
2019-12-23T00:02:50.3578728Z ##[section]Starting: Checkout
2019-12-23T00:02:50.3708211Z ==============================================================================
2019-12-23T00:02:50.3708311Z Task         : Get sources
2019-12-23T00:02:50.3708382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
