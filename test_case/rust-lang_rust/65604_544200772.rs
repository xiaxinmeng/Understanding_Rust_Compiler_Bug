plain
2019-10-19T21:57:12.8404834Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-19T21:57:12.8405310Z 
2019-10-19T21:57:12.8405549Z   git checkout -b <new-branch-name>
2019-10-19T21:57:12.8405807Z 
2019-10-19T21:57:12.8406133Z HEAD is now at fc18016db Auto merge of #65604 - Centril:rollup-bw7ur3d, r=Centril
2019-10-19T21:57:12.8830576Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-19T21:57:12.8944763Z ==============================================================================
2019-10-19T21:57:12.8944863Z Task         : Bash
2019-10-19T21:57:12.8944961Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-19T21:57:45.3261911Z Chocolatey v0.10.15
2019-10-19T21:58:12.0000867Z Installing the following packages:
2019-10-19T21:58:12.0008657Z msys2
2019-10-19T21:58:12.0014571Z By installing you accept licenses for the packages.
2019-10-19T21:58:54.4755375Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-10-19T21:58:54.4756272Z  Unable to connect to the remote server
2019-10-19T21:58:54.4766211Z msys2 not installed. The package was not found with the source(s) listed.
2019-10-19T21:58:54.4766696Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-10-19T21:58:54.4767583Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-10-19T21:58:54.4767987Z If the package version is a prerelease and you didn't specify `--pre`,
2019-10-19T21:58:54.4768749Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-10-19T21:58:54.4769091Z  assistance.
2019-10-19T21:58:54.4885893Z 
2019-10-19T21:58:54.4886508Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-19T21:58:54.4886508Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-19T21:58:54.4886910Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-19T21:58:54.4891429Z 
2019-10-19T21:58:54.4897441Z Failures
2019-10-19T21:58:54.4905867Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-10-19T21:58:54.4906363Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-10-19T21:58:54.4906758Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-10-19T21:58:54.4907116Z If the package version is a prerelease and you didn't specify `--pre`,
2019-10-19T21:58:54.4907974Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-10-19T21:58:54.4908302Z  assistance.
2019-10-19T21:58:54.8408312Z 
2019-10-19T21:58:54.8408312Z 
2019-10-19T21:58:54.8498656Z ##[error]Bash exited with code '1'.
2019-10-19T21:58:54.8712617Z ##[section]Starting: Upload CPU usage statistics
2019-10-19T21:58:54.8834948Z ==============================================================================
2019-10-19T21:58:54.8835204Z Task         : Bash
2019-10-19T21:58:54.8835294Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-19T21:58:55.2086169Z ========================== Starting Command Output ===========================
2019-10-19T21:58:55.2092693Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/00823e2b-916e-4347-848b-cfde4f0eb1bc.sh
2019-10-19T21:58:55.2585905Z /d/a/_temp/00823e2b-916e-4347-848b-cfde4f0eb1bc.sh: line 1: aws: command not found
2019-10-19T21:58:55.2620385Z 
2019-10-19T21:58:55.2639460Z ##[error]Bash exited with code '127'.
2019-10-19T21:58:55.2718194Z ##[section]Starting: Checkout
2019-10-19T21:58:55.2833603Z ==============================================================================
2019-10-19T21:58:55.2833876Z Task         : Get sources
2019-10-19T21:58:55.2833978Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
