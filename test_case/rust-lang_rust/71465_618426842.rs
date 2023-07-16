plain
2020-04-23T14:07:07.7226112Z ========================== Starting Command Output ===========================
2020-04-23T14:07:07.7229307Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8cbcdda4-f04e-42ed-9c27-406dad63da62.sh
2020-04-23T14:07:08.0209161Z 
2020-04-23T14:07:08.0284828Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T14:07:08.0309906Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71465/merge to s
2020-04-23T14:07:08.0321282Z Task         : Get sources
2020-04-23T14:07:08.0321740Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T14:07:08.0322158Z Version      : 1.0.0
2020-04-23T14:07:08.0322478Z Author       : Microsoft
---
2020-04-23T14:07:10.5855526Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T14:07:10.6148093Z ##[command]git config gc.auto 0
2020-04-23T14:07:10.6188073Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T14:07:10.6217261Z ##[command]git config --get-all http.proxy
2020-04-23T14:07:11.1992575Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71465/merge:refs/remotes/pull/71465/merge
---
2020-04-23T14:11:40.8461991Z  ---> 318032b5f0e2
2020-04-23T14:11:40.8462715Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T14:11:40.8463317Z  ---> Using cache
2020-04-23T14:11:40.8463683Z  ---> d44a858fd1ce
2020-04-23T14:11:40.8464532Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T14:11:40.8465555Z  ---> 58b910f50f5a
2020-04-23T14:11:40.8465923Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T14:11:40.8467186Z  ---> Using cache
2020-04-23T14:11:40.8467463Z  ---> ee7702aadba1
---
2020-04-23T14:11:40.9456776Z Looks like docker image is the same as before, not uploading
2020-04-23T14:11:46.1319343Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:11:46.1561749Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:11:46.1585117Z == clock drift check ==
2020-04-23T14:11:46.1590970Z   local time: Thu Apr 23 14:11:46 UTC 2020
2020-04-23T14:11:46.4507191Z   network time: Thu, 23 Apr 2020 14:11:46 GMT
2020-04-23T14:11:46.4563939Z Starting sccache server...
2020-04-23T14:11:46.5268634Z configure: processing command line
2020-04-23T14:11:46.5269006Z configure: 
2020-04-23T14:11:46.5269931Z configure: rust.dist-src        := False
