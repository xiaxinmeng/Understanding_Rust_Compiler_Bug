plain
2019-07-29T14:09:46.1740680Z ==============================================================================
2019-07-29T14:09:46.3688520Z Generating script.
2019-07-29T14:09:46.3737680Z ========================== Starting Command Output ===========================
2019-07-29T14:09:46.3764050Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/b26a89de-4c25-4242-bb16-b34773632f43.sh
2019-07-29T14:14:51.6282490Z fatal: unable to access 'https://github.com/Homebrew/homebrew-cask/': transfer closed with outstanding read data remaining
2019-07-29T14:14:51.6325170Z Error: Fetching /usr/local/Homebrew/Library/Taps/homebrew/homebrew-cask failed!
2019-07-29T14:14:56.8345230Z   https://github.com/Homebrew/brew#donations
2019-07-29T14:15:23.6341390Z Updated 2 taps (caskroom/versions and homebrew/core).
2019-07-29T14:15:23.6341730Z ==> New Formulae
2019-07-29T14:15:23.6342360Z asyncplusplus
---
2019-07-29T14:15:23.6478520Z ==> Deleted Formulae
2019-07-29T14:15:23.6478600Z libggz
2019-07-29T14:15:23.6478750Z libguess
2019-07-29T14:15:23.6478820Z lysp
2019-07-29T14:15:23.6806840Z ##[error]Bash exited with code '1'.
2019-07-29T14:15:23.7090620Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T14:15:23.7095950Z ==============================================================================
2019-07-29T14:15:23.7096060Z Task         : Bash
2019-07-29T14:15:23.7096140Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T14:15:23.8941440Z Script contents:
2019-07-29T14:15:23.8942350Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-29T14:15:23.8967920Z ========================== Starting Command Output ===========================
2019-07-29T14:15:23.8994750Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/01691a29-9c9a-459d-8eee-48bb1cf389bf.sh
2019-07-29T14:15:23.9125020Z /Users/vsts/agent/2.154.3/work/_temp/01691a29-9c9a-459d-8eee-48bb1cf389bf.sh: line 1: aws: command not found
2019-07-29T14:15:23.9213510Z ##[error]Bash exited with code '127'.
2019-07-29T14:15:23.9256250Z ##[section]Starting: Checkout
2019-07-29T14:15:23.9259300Z ==============================================================================
2019-07-29T14:15:23.9259740Z Task         : Get sources
2019-07-29T14:15:23.9259940Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
