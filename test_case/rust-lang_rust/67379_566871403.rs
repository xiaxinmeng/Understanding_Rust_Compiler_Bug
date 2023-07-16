plain
2019-12-18T05:12:49.6266417Z Chocolatey v0.10.15
2019-12-18T05:13:42.1215909Z Installing the following packages:
2019-12-18T05:13:42.1220388Z msys2
2019-12-18T05:13:42.1225904Z By installing you accept licenses for the packages.
2019-12-18T05:15:22.9780668Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-18T05:15:22.9782032Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-18T05:15:22.9787294Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-18T05:15:22.9787946Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-18T05:15:22.9788216Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-18T05:15:22.9788468Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-18T05:15:22.9788873Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-18T05:15:22.9789056Z  assistance.
2019-12-18T05:15:22.9891764Z 
2019-12-18T05:15:22.9892218Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-18T05:15:22.9892218Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-18T05:15:22.9892689Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-18T05:15:22.9896605Z 
2019-12-18T05:15:22.9901749Z Failures
2019-12-18T05:15:22.9908427Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-18T05:15:22.9908914Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-18T05:15:22.9909131Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-18T05:15:22.9909387Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-18T05:15:22.9909785Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-18T05:15:22.9909966Z  assistance.
2019-12-18T05:15:23.3434675Z 
2019-12-18T05:15:23.3434675Z 
2019-12-18T05:15:23.3516912Z ##[error]Bash exited with code '1'.
2019-12-18T05:15:23.3638723Z ##[section]Starting: Checkout
2019-12-18T05:15:23.3743650Z ==============================================================================
2019-12-18T05:15:23.3743750Z Task         : Get sources
2019-12-18T05:15:23.3743826Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
