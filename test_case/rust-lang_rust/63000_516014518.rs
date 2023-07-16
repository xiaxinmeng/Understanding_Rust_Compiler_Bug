plain
2019-07-29T14:18:57.6623740Z ==============================================================================
2019-07-29T14:18:57.8581860Z Generating script.
2019-07-29T14:18:57.8632620Z ========================== Starting Command Output ===========================
2019-07-29T14:18:57.8688660Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/884a37ae-fa12-45f4-acc8-eaf55bf1ec92.sh
2019-07-29T14:25:09.0448170Z error: RPC failed; curl 18 transfer closed with outstanding read data remaining
2019-07-29T14:25:09.0454620Z fatal: the remote end hung up unexpectedly
2019-07-29T14:25:09.0539710Z Error: Fetching /usr/local/Homebrew/Library/Taps/homebrew/homebrew-cask failed!
2019-07-29T14:25:14.3017690Z   https://github.com/Homebrew/brew#donations
2019-07-29T14:25:42.1321900Z Updated 2 taps (caskroom/versions and homebrew/core).
2019-07-29T14:25:42.1322900Z ==> New Formulae
2019-07-29T14:25:42.1323450Z asyncplusplus
---
2019-07-29T14:25:42.1458600Z ==> Deleted Formulae
2019-07-29T14:25:42.1458750Z libggz
2019-07-29T14:25:42.1458830Z libguess
2019-07-29T14:25:42.1458990Z lysp
2019-07-29T14:25:42.1791490Z ##[error]Bash exited with code '1'.
2019-07-29T14:25:42.2065650Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T14:25:42.2070580Z ==============================================================================
2019-07-29T14:25:42.2070700Z Task         : Bash
2019-07-29T14:25:42.2070770Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T14:25:42.3926220Z Script contents:
2019-07-29T14:25:42.3927070Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-29T14:25:42.3952080Z ========================== Starting Command Output ===========================
2019-07-29T14:25:42.3978740Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/bc555231-107f-453b-8479-d3b06170a06a.sh
2019-07-29T14:25:42.4105520Z /Users/vsts/agent/2.154.3/work/_temp/bc555231-107f-453b-8479-d3b06170a06a.sh: line 1: aws: command not found
2019-07-29T14:25:42.4190270Z ##[error]Bash exited with code '127'.
2019-07-29T14:25:42.4274930Z ##[section]Starting: Checkout
2019-07-29T14:25:42.4278010Z ==============================================================================
2019-07-29T14:25:42.4278200Z Task         : Get sources
2019-07-29T14:25:42.4278380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
