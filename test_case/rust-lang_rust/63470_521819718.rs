plain
2019-08-15T22:27:48.4433130Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.155.1/work/_temp/1ca1bbff-70cf-4f29-82c4-27dc5c23f5c9.sh
2019-08-15T22:27:48.4912950Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-08-15T22:27:48.4913640Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-08-15T22:27:48.4913910Z 
2019-08-15T22:27:48.4927440Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com
2019-08-15T22:27:48.5107870Z ##[error]Bash exited with code '6'.
2019-08-15T22:27:48.5386740Z ##[section]Starting: Upload CPU usage statistics
2019-08-15T22:27:48.5391590Z ==============================================================================
2019-08-15T22:27:48.5391690Z Task         : Bash
2019-08-15T22:27:48.5391790Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T22:27:48.7372740Z Script contents:
2019-08-15T22:27:48.7374240Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-08-15T22:27:48.7402300Z ========================== Starting Command Output ===========================
2019-08-15T22:27:48.7428490Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.155.1/work/_temp/d63caf75-f264-4e34-9cbc-80c2be3d782d.sh
2019-08-15T22:27:48.7602840Z /Users/vsts/agent/2.155.1/work/_temp/d63caf75-f264-4e34-9cbc-80c2be3d782d.sh: line 1: aws: command not found
2019-08-15T22:27:48.7690540Z ##[error]Bash exited with code '127'.
2019-08-15T22:27:48.7735790Z ##[section]Starting: Checkout
2019-08-15T22:27:48.7738930Z ==============================================================================
2019-08-15T22:27:48.7739140Z Task         : Get sources
2019-08-15T22:27:48.7739250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
