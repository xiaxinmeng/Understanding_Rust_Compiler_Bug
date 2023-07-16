plain
2019-10-13T11:34:55.9566367Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-13T11:34:55.9566506Z 
2019-10-13T11:34:55.9566636Z   git checkout -b <new-branch-name>
2019-10-13T11:34:55.9567164Z 
2019-10-13T11:34:55.9568177Z HEAD is now at f973dbb26 Auto merge of #65202 - pietroalbini:scriptify-ci-config, r=alexcrichton
2019-10-13T11:34:55.9941989Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-13T11:34:56.0065799Z ==============================================================================
2019-10-13T11:34:56.0065909Z Task         : Bash
2019-10-13T11:34:56.0065980Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-13T11:34:56.7293520Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-13T11:34:56.7694809Z /d/a/_temp
2019-10-13T11:34:56.7783167Z ========================== Starting Command Output ===========================
2019-10-13T11:34:56.7802821Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/5cfb4553-5c08-4566-bb58-77fe814c5c10.sh
2019-10-13T11:34:57.0392110Z /usr/bin/env: 'python3': No such file or directory
2019-10-13T11:34:57.0535809Z ##[error]Bash exited with code '127'.
2019-10-13T11:34:57.0550119Z ##[section]Finishing: Prepare the CI environment
2019-10-13T11:34:57.0728297Z ==============================================================================
2019-10-13T11:34:57.0728414Z Task         : Bash
2019-10-13T11:34:57.0728525Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-13T11:34:57.0728611Z Version      : 3.151.3
---
2019-10-13T11:34:57.3550103Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-13T11:34:57.3904319Z /d/a/_temp
2019-10-13T11:34:57.3983344Z ========================== Starting Command Output ===========================
2019-10-13T11:34:57.4000146Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/aa2921f7-209f-41d0-ba9b-34e6e15b619f.sh
2019-10-13T11:34:57.4446083Z /d/a/_temp/aa2921f7-209f-41d0-ba9b-34e6e15b619f.sh: line 1: aws: command not found
2019-10-13T11:34:57.4502010Z ##[error]Bash exited with code '127'.
2019-10-13T11:34:57.4579257Z ##[section]Starting: Checkout
2019-10-13T11:34:57.4690366Z ==============================================================================
2019-10-13T11:34:57.4690477Z Task         : Get sources
2019-10-13T11:34:57.4690576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
