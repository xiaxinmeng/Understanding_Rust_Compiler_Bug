plain
2020-02-29T00:32:43.1266641Z ========================== Starting Command Output ===========================
2020-02-29T00:32:43.1269383Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ff26a12e-5fce-4f96-96a2-451c7e91a7a7.sh
2020-02-29T00:32:43.1269606Z 
2020-02-29T00:32:43.1274299Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T00:32:43.1293375Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69568/merge to s
2020-02-29T00:32:43.1296585Z Task         : Get sources
2020-02-29T00:32:43.1296816Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T00:32:43.1297041Z Version      : 1.0.0
2020-02-29T00:32:43.1297237Z Author       : Microsoft
---
2020-02-29T00:32:44.4032408Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T00:32:44.4038415Z ##[command]git config gc.auto 0
2020-02-29T00:32:44.4041927Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T00:32:44.4044771Z ##[command]git config --get-all http.proxy
2020-02-29T00:32:44.4050849Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69568/merge:refs/remotes/pull/69568/merge
