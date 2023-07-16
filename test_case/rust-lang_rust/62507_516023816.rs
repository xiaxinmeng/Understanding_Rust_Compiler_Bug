plain
2019-07-29T14:40:55.3952290Z ==============================================================================
2019-07-29T14:40:55.5925900Z Generating script.
2019-07-29T14:40:55.5977060Z ========================== Starting Command Output ===========================
2019-07-29T14:40:55.6003520Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/71d905ed-f2e5-4d5b-acfa-57078743ecb3.sh
2019-07-29T14:46:05.4124170Z fatal: unable to access 'https://github.com/Homebrew/homebrew-cask/': transfer closed with outstanding read data remaining
2019-07-29T14:46:05.4160400Z Error: Fetching /usr/local/Homebrew/Library/Taps/homebrew/homebrew-cask failed!
2019-07-29T14:46:10.4953300Z   https://github.com/Homebrew/brew#donations
2019-07-29T14:46:37.8717170Z Updated 2 taps (caskroom/versions and homebrew/core).
2019-07-29T14:46:37.8718260Z ==> New Formulae
2019-07-29T14:46:37.8719260Z asyncplusplus
---
2019-07-29T14:46:37.9283450Z ==> Deleted Formulae
2019-07-29T14:46:37.9283530Z libggz
2019-07-29T14:46:37.9283780Z libguess
2019-07-29T14:46:37.9283890Z lysp
2019-07-29T14:46:37.9371200Z ##[error]Bash exited with code '1'.
2019-07-29T14:46:37.9600570Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T14:46:37.9606140Z ==============================================================================
2019-07-29T14:46:37.9606280Z Task         : Bash
2019-07-29T14:46:37.9606360Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T14:46:38.1844610Z Script contents:
2019-07-29T14:46:38.1845390Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-29T14:46:38.1845990Z ========================== Starting Command Output ===========================
2019-07-29T14:46:38.1854580Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/83162fbc-7a45-4ee2-8e0b-9b2fc0263f10.sh
2019-07-29T14:46:38.1855700Z /Users/vsts/agent/2.154.3/work/_temp/83162fbc-7a45-4ee2-8e0b-9b2fc0263f10.sh: line 1: aws: command not found
2019-07-29T14:46:38.1858750Z ##[error]Bash exited with code '127'.
2019-07-29T14:46:38.1878600Z ##[section]Starting: Checkout
2019-07-29T14:46:38.1881560Z ==============================================================================
2019-07-29T14:46:38.1881780Z Task         : Get sources
2019-07-29T14:46:38.1881870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
