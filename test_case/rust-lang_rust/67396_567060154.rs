plain
2019-12-18T14:39:34.9711261Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-18T14:39:34.9711640Z 
2019-12-18T14:39:34.9712205Z   git checkout -b <new-branch-name>
2019-12-18T14:39:34.9712639Z 
2019-12-18T14:39:34.9712986Z HEAD is now at f2df3f706 Auto merge of #67396 - Mark-Simulacrum:rollup-85lxz7h, r=Mark-Simulacrum
2019-12-18T14:39:35.0080834Z ##[section]Starting: Setup environment
2019-12-18T14:39:35.0185896Z ==============================================================================
2019-12-18T14:39:35.0186002Z Task         : Bash
2019-12-18T14:39:35.0186073Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-18T14:39:48.8288541Z Chocolatey v0.10.15
2019-12-18T14:40:22.8063983Z Installing the following packages:
2019-12-18T14:40:22.8069194Z msys2
2019-12-18T14:40:22.8073058Z By installing you accept licenses for the packages.
2019-12-18T14:42:03.4933945Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-18T14:42:03.4934993Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-18T14:42:03.4942177Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-18T14:42:03.4942663Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-18T14:42:03.4943146Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-18T14:42:03.4943915Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-18T14:42:03.4944639Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-18T14:42:03.4945508Z  assistance.
2019-12-18T14:42:03.5053892Z 
2019-12-18T14:42:03.5054493Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-18T14:42:03.5054493Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-18T14:42:03.5055102Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-18T14:42:03.5058765Z 
2019-12-18T14:42:03.5064937Z Failures
2019-12-18T14:42:03.5072164Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-18T14:42:03.5072847Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-18T14:42:03.5073340Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-18T14:42:03.5073777Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-18T14:42:03.5074929Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-18T14:42:03.5075282Z  assistance.
2019-12-18T14:42:03.8728106Z 
2019-12-18T14:42:03.8728106Z 
2019-12-18T14:42:03.8807137Z ##[error]Bash exited with code '1'.
2019-12-18T14:42:03.8934423Z ##[section]Starting: Checkout
2019-12-18T14:42:03.9045784Z ==============================================================================
2019-12-18T14:42:03.9045899Z Task         : Get sources
2019-12-18T14:42:03.9045982Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
