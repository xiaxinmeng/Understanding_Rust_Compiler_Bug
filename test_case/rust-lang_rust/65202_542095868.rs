plain
2019-10-15T07:58:00.5257415Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-15T07:58:00.5257479Z 
2019-10-15T07:58:00.5257529Z   git checkout -b <new-branch-name>
2019-10-15T07:58:00.5257625Z 
2019-10-15T07:58:00.5257888Z HEAD is now at eda9abe5d Auto merge of #65202 - pietroalbini:scriptify-ci-config, r=<try>
2019-10-15T07:58:00.5641772Z ##[section]Finishing: Checkout
2019-10-15T07:58:00.5807791Z ##[section]Starting: Copy python.exe to python3.exe
2019-10-15T07:58:00.5928015Z Task         : Bash
2019-10-15T07:58:00.5928128Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T07:58:00.5928202Z Version      : 3.151.3
2019-10-15T07:58:00.5928284Z Author       : Microsoft Corporation
2019-10-15T07:58:00.5928284Z Author       : Microsoft Corporation
2019-10-15T07:58:00.5928366Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-10-15T07:58:00.5928475Z ==============================================================================
2019-10-15T07:58:00.8993499Z Generating script.
2019-10-15T07:58:00.9215325Z Script contents:
2019-10-15T07:58:00.9215433Z cp $(which python) $(dirname $(which python))/python3.exe
2019-10-15T07:58:01.1295694Z /d/a/_temp
2019-10-15T07:58:01.1370674Z ========================== Starting Command Output ===========================
2019-10-15T07:58:01.1380734Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ea5fd6c6-c4e3-43c0-9034-d384ad980016.sh
2019-10-15T07:58:01.1380734Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ea5fd6c6-c4e3-43c0-9034-d384ad980016.sh
2019-10-15T07:58:01.6998205Z ##[section]Finishing: Copy python.exe to python3.exe
2019-10-15T07:58:01.7140784Z ==============================================================================
2019-10-15T07:58:01.7140895Z Task         : Bash
2019-10-15T07:58:01.7141000Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T07:58:01.7141080Z Version      : 3.151.3
---
2019-10-15T07:58:02.4274910Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-15T07:58:02.4723434Z /d/a/_temp
2019-10-15T07:58:02.4806888Z ========================== Starting Command Output ===========================
2019-10-15T07:58:02.4816018Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/3cadfcd3-8f88-4020-bcc0-b010d541ce75.sh
2019-10-15T07:58:04.6853089Z ==== Show the current environment ====
2019-10-15T07:58:04.6853711Z bash: src/ciscriptsdump-environment.sh: No such file or directory
2019-10-15T07:58:04.6853956Z ==== failure: Show the current environment ====
2019-10-15T07:58:04.6854140Z exit code: 127
2019-10-15T07:58:04.7086538Z ##[error]Bash exited with code '127'.
2019-10-15T07:58:04.7101255Z ##[section]Finishing: Prepare the CI environment
2019-10-15T07:58:04.7330386Z ==============================================================================
2019-10-15T07:58:04.7330524Z Task         : Bash
2019-10-15T07:58:04.7330609Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T07:58:04.7330718Z Version      : 3.151.3
---
2019-10-15T07:58:05.1527594Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-15T07:58:05.1528595Z /d/a/_temp
2019-10-15T07:58:05.1530398Z ========================== Starting Command Output ===========================
2019-10-15T07:58:05.1531504Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/860fbe7e-4d95-4c76-90d2-0e75f338284b.sh
2019-10-15T07:58:05.1531642Z /d/a/_temp/860fbe7e-4d95-4c76-90d2-0e75f338284b.sh: line 1: aws: command not found
2019-10-15T07:58:05.1532120Z ##[error]Bash exited with code '127'.
2019-10-15T07:58:05.1554578Z ##[section]Starting: Checkout
2019-10-15T07:58:05.1670897Z ==============================================================================
2019-10-15T07:58:05.1671017Z Task         : Get sources
2019-10-15T07:58:05.1671110Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
