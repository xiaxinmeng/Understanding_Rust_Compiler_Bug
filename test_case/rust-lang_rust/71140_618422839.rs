plain
2020-04-23T14:05:11.4546618Z ========================== Starting Command Output ===========================
2020-04-23T14:05:11.4564444Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e1e34a7b-d0e7-429e-a458-d910909cc4a7.sh
2020-04-23T14:05:11.4777655Z 
2020-04-23T14:05:11.4839402Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T14:05:11.4857947Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71140/merge to s
2020-04-23T14:05:11.4861243Z Task         : Get sources
2020-04-23T14:05:11.5001599Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T14:05:11.5002011Z Version      : 1.0.0
2020-04-23T14:05:11.5002172Z Author       : Microsoft
---
2020-04-23T14:05:12.6643976Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T14:05:12.6649191Z ##[command]git config gc.auto 0
2020-04-23T14:05:12.6652946Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T14:05:12.6656279Z ##[command]git config --get-all http.proxy
2020-04-23T14:05:12.6665907Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71140/merge:refs/remotes/pull/71140/merge
---
2020-04-23T14:07:25.6531084Z  ---> 318032b5f0e2
2020-04-23T14:07:25.6532194Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T14:07:25.6534222Z  ---> Using cache
2020-04-23T14:07:25.6534975Z  ---> d44a858fd1ce
2020-04-23T14:07:25.6536061Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T14:07:25.6538481Z  ---> 58b910f50f5a
2020-04-23T14:07:25.6538882Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T14:07:25.6541935Z  ---> Using cache
2020-04-23T14:07:25.6542712Z  ---> ee7702aadba1
---
2020-04-23T14:07:25.6916417Z Looks like docker image is the same as before, not uploading
2020-04-23T14:07:31.7125202Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:07:31.7381241Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:07:31.7409060Z == clock drift check ==
2020-04-23T14:07:31.7425496Z   local time: Thu Apr 23 14:07:31 UTC 2020
2020-04-23T14:07:31.8823485Z   network time: Thu, 23 Apr 2020 14:07:31 GMT
2020-04-23T14:07:31.8847495Z Starting sccache server...
2020-04-23T14:07:31.9577841Z configure: processing command line
2020-04-23T14:07:31.9579637Z configure: 
2020-04-23T14:07:31.9580797Z configure: rust.dist-src        := False
