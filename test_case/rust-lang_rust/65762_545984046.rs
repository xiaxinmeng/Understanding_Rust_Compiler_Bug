plain
2019-10-24T15:49:19.7952981Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-24T15:49:19.7953039Z 
2019-10-24T15:49:19.7953131Z   git checkout -b <new-branch-name>
2019-10-24T15:49:19.7953182Z 
2019-10-24T15:49:19.7953285Z HEAD is now at bc8d760ee Auto merge of #65762 - mati865:msys2-bug, r=pietroalbini
2019-10-24T15:49:19.8356088Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-24T15:49:19.8463820Z ==============================================================================
2019-10-24T15:49:19.8463945Z Task         : Bash
2019-10-24T15:49:19.8464031Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-24T15:53:45.8332470Z installing mingw-w64-i686-libiconv...
2019-10-24T15:53:45.9142091Z installing mingw-w64-i686-gettext...
2019-10-24T15:53:48.6938642Z installing mingw-w64-i686-p11-kit...
2019-10-24T15:53:48.7482442Z installing mingw-w64-i686-ca-certificates...
2019-10-24T15:53:50.2015481Z error: 'mingw-w64-x86_64-ca-certificates-20180409-1-any.pkg.tar.xz': could not find or read package
2019-10-24T15:53:50.2160799Z loading packages...
2019-10-24T15:53:50.2301382Z rm: cannot remove 'mingw-w64-x86_64-ca-certificates-20180409-1-any.pkg.tar.xz': No such file or directory
2019-10-24T15:53:50.2311968Z 
2019-10-24T15:53:50.2421010Z ##[error]Bash exited with code '1'.
2019-10-24T15:53:50.2463057Z ##[section]Finishing: Download working ca-certificates for msys
2019-10-24T15:53:50.2693832Z ==============================================================================
2019-10-24T15:53:50.2693941Z Task         : Bash
2019-10-24T15:53:50.2694032Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-24T15:53:50.2694110Z Version      : 3.159.3
---
2019-10-24T15:53:50.5466014Z ========================== Starting Command Output ===========================
2019-10-24T15:53:50.5470175Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/3a2dacf4-f7a1-4a82-827e-ebcb2fbf2b6b.sh
2019-10-24T15:53:50.5819076Z /d/a/_temp/3a2dacf4-f7a1-4a82-827e-ebcb2fbf2b6b.sh: line 1: aws: command not found
2019-10-24T15:53:50.5826473Z 
2019-10-24T15:53:50.5854968Z ##[error]Bash exited with code '127'.
2019-10-24T15:53:50.5926258Z ##[section]Starting: Checkout
2019-10-24T15:53:50.6019253Z ==============================================================================
2019-10-24T15:53:50.6019330Z Task         : Get sources
2019-10-24T15:53:50.6019413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
