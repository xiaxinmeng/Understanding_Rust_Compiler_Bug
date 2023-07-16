plain
2020-04-23T14:00:42.5709458Z ========================== Starting Command Output ===========================
2020-04-23T14:00:42.5711781Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e6e8ccf-450d-4e98-b473-f7210a98794b.sh
2020-04-23T14:00:42.5712040Z 
2020-04-23T14:00:42.5715780Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T14:00:42.5734871Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71467/merge to s
2020-04-23T14:00:42.5738123Z Task         : Get sources
2020-04-23T14:00:42.5738404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T14:00:42.5738694Z Version      : 1.0.0
2020-04-23T14:00:42.5738885Z Author       : Microsoft
---
2020-04-23T14:00:43.8094792Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T14:00:43.8108786Z ##[command]git config gc.auto 0
2020-04-23T14:00:43.8122691Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T14:00:43.8128922Z ##[command]git config --get-all http.proxy
2020-04-23T14:00:44.8078142Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71467/merge:refs/remotes/pull/71467/merge
---
2020-04-23T14:04:31.8343929Z  ---> 318032b5f0e2
2020-04-23T14:04:31.8345275Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T14:04:31.8355158Z  ---> Using cache
2020-04-23T14:04:31.8355798Z  ---> d44a858fd1ce
2020-04-23T14:04:31.8357494Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T14:04:31.8363886Z  ---> 58b910f50f5a
2020-04-23T14:04:31.8364096Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T14:04:31.8370799Z  ---> Using cache
2020-04-23T14:04:31.8371171Z  ---> ee7702aadba1
---
2020-04-23T14:04:31.8749175Z Looks like docker image is the same as before, not uploading
2020-04-23T14:04:39.3084517Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:04:39.3427596Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T14:04:39.3463757Z == clock drift check ==
2020-04-23T14:04:39.3475290Z   local time: Thu Apr 23 14:04:39 UTC 2020
2020-04-23T14:04:39.6168518Z   network time: Thu, 23 Apr 2020 14:04:39 GMT
2020-04-23T14:04:39.6195101Z Starting sccache server...
2020-04-23T14:04:39.7048789Z configure: processing command line
2020-04-23T14:04:39.7049075Z configure: 
2020-04-23T14:04:39.7049927Z configure: rust.dist-src        := False
