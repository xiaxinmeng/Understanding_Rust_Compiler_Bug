plain
2019-09-18T10:31:06.8081417Z do so (now or later) by using -b with the checkout command again. Example:
2019-09-18T10:31:06.8081869Z 
2019-09-18T10:31:06.8082359Z   git checkout -b <new-branch-name>
2019-09-18T10:31:06.8083183Z 
2019-09-18T10:31:06.8083698Z HEAD is now at 281b66657 Auto merge of #64553 - alexcrichton:windows-bash-install-scripts, r=Mark-Simulacrum
2019-09-18T10:31:06.8492295Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-09-18T10:31:06.8615551Z ==============================================================================
2019-09-18T10:31:06.8615646Z Task         : Bash
2019-09-18T10:31:06.8615736Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-09-18T10:32:14.4350151Z By installing you accept licenses for the packages.
2019-09-18T10:32:20.2039778Z 
2019-09-18T10:32:20.2040523Z msys2 v20180531.0.0 [Approved]
2019-09-18T10:32:20.2159241Z msys2 package files install completed. Performing other installation steps.
2019-09-18T10:32:23.7535253Z Installing to: D:\a/msys2
2019-09-18T10:32:24.4707124Z Extracting 64-bit C:\ProgramData\chocolatey\lib\msys2\tools\msys2-base-x86_64-20180531.tar.xz to D:\a/msys2...
2019-09-18T10:32:31.8483362Z D:\a/msys2
2019-09-18T10:32:31.8553019Z Extracting D:\a\msys2\msys2-base-x86_64-20180531.tar to D:\a/msys2...
2019-09-18T10:32:52.5582295Z D:\a/msys2
2019-09-18T10:33:01.1552236Z Repeating system update until there are no more updates or max 5 iterations
2019-09-18T10:33:01.1566856Z 
2019-09-18T10:33:01.1567010Z ================= SYSTEM UPDATE 1 =================
2019-09-18T10:33:01.1567061Z 
---
2019-09-18T10:34:09.1671480Z   Software installed to 'D:\a/msys2'
2019-09-18T10:34:09.2171915Z 
2019-09-18T10:34:09.2172382Z Chocolatey installed 1/1 packages. 
2019-09-18T10:34:09.2172623Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-09-18T10:34:19.7969957Z The STDIO streams did not close within 10 seconds of the exit event from process 'C:\Program Files\Git\bin\bash.exe'. This may indicate a child process inherited the STDIO streams and has not yet exited.
2019-09-18T10:34:24.8190055Z ##[section]Starting: Install msys2 base deps
2019-09-18T10:34:24.8321564Z ==============================================================================
2019-09-18T10:34:24.8321662Z Task         : Bash
2019-09-18T10:34:24.8321761Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-09-18T10:34:24.8321761Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-09-18T10:34:24.8321990Z Version      : 3.151.3
2019-09-18T10:34:24.8322080Z Author       : Microsoft Corporation
2019-09-18T10:34:24.8322158Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-09-18T10:34:24.8322269Z ==============================================================================
2019-09-18T10:34:25.0894482Z Generating script.
2019-09-18T10:34:25.0922231Z Script contents:
2019-09-18T10:34:25.0934410Z pacman -S --noconfirm --needed base-devel ca-certificates make diffutils tar
2019-09-18T10:34:25.0998699Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc -c pwd
2019-09-18T10:34:25.1363327Z ========================== Starting Command Output ===========================
2019-09-18T10:34:25.1381857Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/49f55fd4-a66e-4718-9053-5b809e06d8fb.sh
2019-09-18T10:34:25.8359222Z :: There are 56 members in group base-devel:
2019-09-18T10:34:25.8359364Z :: Repository msys
---
2019-09-18T10:35:35.2886945Z 
2019-09-18T10:35:35.2887004Z Scanning the drive for archives:
2019-09-18T10:35:35.2887089Z 1 file, 188813 bytes (185 KiB)
2019-09-18T10:35:35.2887130Z 
2019-09-18T10:35:35.2887199Z Extracting archive: ninja.zip
2019-09-18T10:35:35.2934170Z --
2019-09-18T10:35:35.2934262Z Path = ninja.zip
2019-09-18T10:35:35.2934428Z Physical Size = 188813
2019-09-18T10:35:35.2934579Z 
2019-09-18T10:35:35.2934640Z Everything is Ok
2019-09-18T10:35:35.2934676Z 
---
2019-09-18T10:35:35.6208355Z git config --replace-all --global core.autocrlf false
2019-09-18T10:35:35.6270137Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc -c pwd
2019-09-18T10:35:35.6590739Z /d/a/_temp
2019-09-18T10:35:35.6660398Z ========================== Starting Command Output ===========================
2019-09-18T10:35:35.6681234Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/3aafc9d7-2ec0-42d6-a9b0-b7d3e880780d.sh
2019-09-18T10:35:35.7327172Z error: could not lock config file D:/a/msys2/home/VssAdministrator/.gitconfig: No such file or directory
2019-09-18T10:35:35.7480438Z ##[error]Bash exited with code '255'.
2019-09-18T10:35:35.7590085Z ##[section]Starting: Upload CPU usage statistics
2019-09-18T10:35:35.7704986Z ==============================================================================
2019-09-18T10:35:35.7705096Z Task         : Bash
2019-09-18T10:35:35.7705168Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-09-18T10:35:36.0379485Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$CI_JOB_NAME.csv
2019-09-18T10:35:36.0443355Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc -c pwd
2019-09-18T10:35:36.0777509Z /d/a/_temp
2019-09-18T10:35:36.0827212Z ========================== Starting Command Output ===========================
2019-09-18T10:35:36.0846615Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/987d4b7e-2917-4dff-981c-6a0d555f1618.sh
2019-09-18T10:35:36.1238580Z /d/a/_temp/987d4b7e-2917-4dff-981c-6a0d555f1618.sh: line 1: aws: command not found
2019-09-18T10:35:36.1282856Z ##[error]Bash exited with code '127'.
2019-09-18T10:35:36.1354443Z ##[section]Starting: Checkout
2019-09-18T10:35:36.1465943Z ==============================================================================
2019-09-18T10:35:36.1466042Z Task         : Get sources
2019-09-18T10:35:36.1466294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
