plain
2020-04-23T14:03:41.8282634Z ========================== Starting Command Output ===========================
2020-04-23T14:03:41.8285967Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dc21d24e-0376-4f49-9f37-27770c10b358.sh
2020-04-23T14:03:41.8286371Z 
2020-04-23T14:03:41.8290592Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T14:03:41.8307494Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-23T14:03:41.8310507Z Task         : Get sources
2020-04-23T14:03:41.8310787Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T14:03:41.8311076Z Version      : 1.0.0
2020-04-23T14:03:41.8311263Z Author       : Microsoft
---
2020-04-23T14:03:43.0876928Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T14:03:43.0883231Z ##[command]git config gc.auto 0
2020-04-23T14:03:43.0886769Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T14:03:43.0889905Z ##[command]git config --get-all http.proxy
2020-04-23T14:03:43.0896706Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71439/merge:refs/remotes/pull/71439/merge
---
2020-04-23T14:07:13.0699680Z  ---> 318032b5f0e2
2020-04-23T14:07:13.0700430Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T14:07:13.0705869Z  ---> Using cache
2020-04-23T14:07:13.0706403Z  ---> d44a858fd1ce
2020-04-23T14:07:13.0707588Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T14:07:13.0712589Z  ---> 58b910f50f5a
2020-04-23T14:07:13.0712980Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T14:07:13.0717134Z  ---> Using cache
2020-04-23T14:07:13.0717518Z  ---> ee7702aadba1
---
2020-04-23T14:07:13.1103865Z Looks like docker image is the same as before, not uploading
2020-04-23T14:07:21.2128420Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:07:21.2485314Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:07:21.2516243Z == clock drift check ==
2020-04-23T14:07:21.2529903Z   local time: Thu Apr 23 14:07:21 UTC 2020
2020-04-23T14:07:21.3359714Z   network time: Thu, 23 Apr 2020 14:07:21 GMT
2020-04-23T14:07:21.3385472Z Starting sccache server...
2020-04-23T14:07:21.4229340Z configure: processing command line
2020-04-23T14:07:21.4229949Z configure: 
2020-04-23T14:07:21.4230945Z configure: rust.dist-src        := False
