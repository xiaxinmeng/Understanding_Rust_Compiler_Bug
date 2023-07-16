plain
2019-07-29T10:12:11.0785010Z ==============================================================================
2019-07-29T10:12:11.2633420Z Generating script.
2019-07-29T10:12:11.2680420Z ========================== Starting Command Output ===========================
2019-07-29T10:12:11.2706170Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/d763b023-8f12-400e-b0e7-49b93350127b.sh
2019-07-29T10:18:06.5302210Z error: RPC failed; curl 18 transfer closed with outstanding read data remaining
2019-07-29T10:18:06.5318420Z fatal: the remote end hung up unexpectedly
2019-07-29T10:18:06.5389450Z Error: Fetching /usr/local/Homebrew/Library/Taps/homebrew/homebrew-cask failed!
2019-07-29T10:18:11.8529650Z   https://github.com/Homebrew/brew#donations
2019-07-29T10:18:38.3050330Z Updated 2 taps (caskroom/versions and homebrew/core).
2019-07-29T10:18:38.3051650Z ==> New Formulae
2019-07-29T10:18:38.3052380Z asyncplusplus
---
2019-07-29T10:18:38.3183530Z ==> Deleted Formulae
2019-07-29T10:18:38.3183600Z libggz
2019-07-29T10:18:38.3183760Z libguess
2019-07-29T10:18:38.3183820Z lysp
2019-07-29T10:18:38.3555150Z ##[error]Bash exited with code '1'.
2019-07-29T10:18:38.3775940Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T10:18:38.3780800Z ==============================================================================
2019-07-29T10:18:38.3780910Z Task         : Bash
2019-07-29T10:18:38.3781000Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T10:18:38.5718160Z Script contents:
2019-07-29T10:18:38.5721790Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-29T10:18:38.5750850Z ========================== Starting Command Output ===========================
2019-07-29T10:18:38.5777900Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/b49b91c1-74f7-4d5e-8510-fc861828ebd2.sh
2019-07-29T10:18:38.5899630Z /Users/vsts/agent/2.154.3/work/_temp/b49b91c1-74f7-4d5e-8510-fc861828ebd2.sh: line 1: aws: command not found
2019-07-29T10:18:38.5986310Z ##[error]Bash exited with code '127'.
2019-07-29T10:18:38.6030470Z ##[section]Starting: Checkout
2019-07-29T10:18:38.6033810Z ==============================================================================
2019-07-29T10:18:38.6033940Z Task         : Get sources
2019-07-29T10:18:38.6034120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
