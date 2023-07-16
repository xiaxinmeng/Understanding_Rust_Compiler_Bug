plain
2019-07-29T14:28:54.8658220Z ==============================================================================
2019-07-29T14:28:55.0647140Z Generating script.
2019-07-29T14:28:55.0781520Z ========================== Starting Command Output ===========================
2019-07-29T14:28:55.0879930Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/c047ec1c-e132-43b9-aaa0-ffd570703580.sh
2019-07-29T14:37:10.4458420Z error: RPC failed; curl 18 transfer closed with outstanding read data remaining
2019-07-29T14:37:10.4465060Z fatal: the remote end hung up unexpectedly
2019-07-29T14:37:10.4549190Z Error: Fetching /usr/local/Homebrew/Library/Taps/homebrew/homebrew-cask failed!
2019-07-29T14:37:15.6178270Z   https://github.com/Homebrew/brew#donations
2019-07-29T14:37:44.6307910Z Updated 2 taps (caskroom/versions and homebrew/core).
2019-07-29T14:37:44.6309260Z ==> New Formulae
2019-07-29T14:37:44.6310400Z asyncplusplus
---
2019-07-29T14:37:44.6448030Z ==> Deleted Formulae
2019-07-29T14:37:44.6448550Z libggz
2019-07-29T14:37:44.6448670Z libguess
2019-07-29T14:37:44.6448840Z lysp
2019-07-29T14:37:44.6814510Z ##[error]Bash exited with code '1'.
2019-07-29T14:37:44.7058410Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T14:37:44.7064130Z ==============================================================================
2019-07-29T14:37:44.7064250Z Task         : Bash
2019-07-29T14:37:44.7064330Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T14:37:44.9044350Z Script contents:
2019-07-29T14:37:44.9045210Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-29T14:37:44.9071660Z ========================== Starting Command Output ===========================
2019-07-29T14:37:44.9097780Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/a70fe447-183f-4161-814d-87cf6c02956b.sh
2019-07-29T14:37:44.9215050Z /Users/vsts/agent/2.154.3/work/_temp/a70fe447-183f-4161-814d-87cf6c02956b.sh: line 1: aws: command not found
2019-07-29T14:37:44.9304020Z ##[error]Bash exited with code '127'.
2019-07-29T14:37:44.9355100Z ##[section]Starting: Checkout
2019-07-29T14:37:44.9358260Z ==============================================================================
2019-07-29T14:37:44.9358880Z Task         : Get sources
2019-07-29T14:37:44.9358980Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
