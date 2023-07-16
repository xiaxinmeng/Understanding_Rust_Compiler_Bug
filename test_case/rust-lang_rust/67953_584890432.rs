plain
2020-02-11T22:19:07.0432306Z ========================== Starting Command Output ===========================
2020-02-11T22:19:07.0729988Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dbe1b047-5df0-4e10-8107-882e5330cd6b.sh
2020-02-11T22:19:07.0730091Z 
2020-02-11T22:19:07.0733115Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T22:19:07.0739081Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67953/merge to s
2020-02-11T22:19:07.0741139Z Task         : Get sources
2020-02-11T22:19:07.0741177Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T22:19:07.0741213Z Version      : 1.0.0
2020-02-11T22:19:07.0741292Z Author       : Microsoft
---
2020-02-11T22:19:07.9397962Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T22:19:07.9493899Z ##[command]git config gc.auto 0
2020-02-11T22:19:07.9564553Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T22:19:07.9612781Z ##[command]git config --get-all http.proxy
2020-02-11T22:19:07.9753157Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67953/merge:refs/remotes/pull/67953/merge
