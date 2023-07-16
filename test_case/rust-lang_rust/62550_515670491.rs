plain
2019-07-27T09:25:00.7814693Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T09:25:00.7815773Z 
2019-07-27T09:25:00.7816887Z   git checkout -b <new-branch-name>
2019-07-27T09:25:00.7818023Z 
2019-07-27T09:25:00.7819257Z HEAD is now at 42abf7a85 Auto merge of #62550 - Centril:rest-patterns, r=petrochenkov
2019-07-27T09:25:00.7955835Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T09:25:00.7959068Z ==============================================================================
2019-07-27T09:25:00.7959175Z Task         : Bash
2019-07-27T09:25:00.7959245Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T09:30:18.9620290Z Script contents:
2019-07-27T09:30:18.9620962Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-27T09:30:18.9640671Z ========================== Starting Command Output ===========================
2019-07-27T09:30:18.9660810Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cedbf029-ad73-4857-b06d-b927673694a9.sh
2019-07-27T09:30:18.9752087Z /home/vsts/work/_temp/cedbf029-ad73-4857-b06d-b927673694a9.sh: line 1: aws: command not found
2019-07-27T09:30:18.9798183Z ##[error]Bash exited with code '127'.
2019-07-27T09:30:18.9825485Z ##[section]Starting: Checkout
2019-07-27T09:30:18.9827712Z ==============================================================================
2019-07-27T09:30:18.9827828Z Task         : Get sources
2019-07-27T09:30:18.9827928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
