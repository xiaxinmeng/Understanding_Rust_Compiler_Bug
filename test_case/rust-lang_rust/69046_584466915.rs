plain
2020-02-11T03:25:38.2618433Z ========================== Starting Command Output ===========================
2020-02-11T03:25:38.2634109Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a39475b5-2a9f-4986-8bdd-df1a7bc0ebae.sh
2020-02-11T03:25:38.2872398Z 
2020-02-11T03:25:38.2907120Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T03:25:38.2912211Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69046/merge to s
2020-02-11T03:25:38.2914139Z Task         : Get sources
2020-02-11T03:25:38.2914173Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T03:25:38.2914209Z Version      : 1.0.0
2020-02-11T03:25:38.2914240Z Author       : Microsoft
---
2020-02-11T03:25:39.2945517Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T03:25:39.3024434Z ##[command]git config gc.auto 0
2020-02-11T03:25:39.3101508Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T03:25:39.3167893Z ##[command]git config --get-all http.proxy
2020-02-11T03:25:39.3292581Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69046/merge:refs/remotes/pull/69046/merge
