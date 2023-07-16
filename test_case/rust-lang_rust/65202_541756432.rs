plain
2019-10-14T15:42:39.1411382Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-14T15:42:39.1411821Z 
2019-10-14T15:42:39.1412362Z   git checkout -b <new-branch-name>
2019-10-14T15:42:39.1413401Z 
2019-10-14T15:42:39.1414956Z HEAD is now at a103c61b9 Auto merge of #65202 - pietroalbini:scriptify-ci-config, r=alexcrichton
2019-10-14T15:42:39.1774570Z ##[section]Finishing: Checkout
2019-10-14T15:42:39.1906405Z ##[section]Starting: Copy python.exe to python3.exe
2019-10-14T15:42:39.2054625Z Task         : Bash
2019-10-14T15:42:39.2054698Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-14T15:42:39.2054786Z Version      : 3.151.3
2019-10-14T15:42:39.2054847Z Author       : Microsoft Corporation
2019-10-14T15:42:39.2054847Z Author       : Microsoft Corporation
2019-10-14T15:42:39.2054944Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-10-14T15:42:39.2055051Z ==============================================================================
2019-10-14T15:42:39.4543072Z Generating script.
2019-10-14T15:42:39.4753647Z Script contents:
2019-10-14T15:42:39.4753754Z cp $(which python) $(dirname $(which python))/python3.exe
2019-10-14T15:42:39.5185261Z /d/a/_temp
2019-10-14T15:42:39.5257973Z ========================== Starting Command Output ===========================
2019-10-14T15:42:39.5266878Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ff005c95-8054-45b4-8d20-9a4c14409d20.sh
2019-10-14T15:42:39.5266878Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ff005c95-8054-45b4-8d20-9a4c14409d20.sh
2019-10-14T15:42:39.9225103Z ##[section]Finishing: Copy python.exe to python3.exe
2019-10-14T15:42:39.9354030Z ==============================================================================
2019-10-14T15:42:39.9354118Z Task         : Bash
2019-10-14T15:42:39.9354209Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-14T15:42:39.9354277Z Version      : 3.151.3
---
2019-10-14T15:42:40.6946372Z ========================== Starting Command Output ===========================
2019-10-14T15:42:40.6974190Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/538bcdc1-6201-47e5-a9b8-9bf6374e2be5.sh
2019-10-14T15:42:42.5321686Z ==== Show the current environment ====
2019-10-14T15:42:42.6037975Z Traceback (most recent call last):
2019-10-14T15:42:42.6044451Z   File "src/ci/prepare.py", line 93, in <module>
2019-10-14T15:42:42.6065675Z     run(Build())
2019-10-14T15:42:42.6065818Z   File "src/ci/prepare.py", line 9, in run
2019-10-14T15:42:42.6065915Z     b.step("dump-environment.sh", "Show the current environment")
2019-10-14T15:42:42.6066030Z   File "src/ci/prepare.py", line 63, in step
2019-10-14T15:42:42.6066120Z     stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
2019-10-14T15:42:42.6066247Z   File "C:\hostedtoolcache\windows\Python\3.6.8\x64\lib\subprocess.py", line 729, in __init__
2019-10-14T15:42:42.6066358Z     restore_signals, start_new_session)
2019-10-14T15:42:42.6067130Z   File "C:\hostedtoolcache\windows\Python\3.6.8\x64\lib\subprocess.py", line 1017, in _execute_child
2019-10-14T15:42:42.6067297Z     startupinfo)
2019-10-14T15:42:42.6067385Z OSError: [WinError 193] %1 is not a valid Win32 application
2019-10-14T15:42:42.6363394Z ##[error]Bash exited with code '1'.
2019-10-14T15:42:42.6572272Z ##[section]Finishing: Prepare the CI environment
2019-10-14T15:42:42.7073475Z ==============================================================================
2019-10-14T15:42:42.7073584Z Task         : Bash
2019-10-14T15:42:42.7073655Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-14T15:42:42.7073736Z Version      : 3.151.3
---
2019-10-14T15:42:43.1372960Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-14T15:42:43.2024724Z /d/a/_temp
2019-10-14T15:42:43.2554441Z ========================== Starting Command Output ===========================
2019-10-14T15:42:43.2555216Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/bb4006b7-6135-4204-89cc-c438644dca5b.sh
2019-10-14T15:42:43.3650097Z /d/a/_temp/bb4006b7-6135-4204-89cc-c438644dca5b.sh: line 1: aws: command not found
2019-10-14T15:42:43.3796243Z ##[error]Bash exited with code '127'.
2019-10-14T15:42:43.3860654Z ##[section]Starting: Checkout
2019-10-14T15:42:43.3993917Z ==============================================================================
2019-10-14T15:42:43.3994044Z Task         : Get sources
2019-10-14T15:42:43.3994302Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
