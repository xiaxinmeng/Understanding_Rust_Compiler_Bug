plain
2019-12-21T17:42:24.8711604Z Chocolatey v0.10.15
2019-12-21T17:43:17.9400862Z Installing the following packages:
2019-12-21T17:43:17.9405645Z msys2
2019-12-21T17:43:17.9411074Z By installing you accept licenses for the packages.
2019-12-21T17:44:58.8526742Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-21T17:44:58.8527560Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-21T17:44:58.8534440Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:44:58.8534928Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:44:58.8535491Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:44:58.8536168Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:44:58.8536841Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:44:58.8537179Z  assistance.
2019-12-21T17:44:58.8626916Z 
2019-12-21T17:44:58.8627263Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:44:58.8627263Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-21T17:44:58.8627434Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-21T17:44:58.8631555Z 
2019-12-21T17:44:58.8635328Z Failures
2019-12-21T17:44:58.8641904Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-21T17:44:58.8642009Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-21T17:44:58.8642113Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-21T17:44:58.8642364Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-21T17:44:58.8642699Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-21T17:44:58.8642789Z  assistance.
2019-12-21T17:44:59.2111558Z 
2019-12-21T17:44:59.2111558Z 
2019-12-21T17:44:59.2192836Z ##[error]Bash exited with code '1'.
2019-12-21T17:44:59.2309310Z ##[section]Starting: Checkout
2019-12-21T17:44:59.2415231Z ==============================================================================
2019-12-21T17:44:59.2415308Z Task         : Get sources
2019-12-21T17:44:59.2415399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
