plain
2020-03-12T00:25:38.2930721Z ========================== Starting Command Output ===========================
2020-03-12T00:25:38.2933521Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ae0ccf62-8a18-4c6f-87fd-d4cbe299a33f.sh
2020-03-12T00:25:38.2933782Z 
2020-03-12T00:25:38.2938247Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-12T00:25:38.2958447Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-03-12T00:25:38.2962691Z Task         : Get sources
2020-03-12T00:25:38.2963002Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T00:25:38.2963282Z Version      : 1.0.0
2020-03-12T00:25:38.2963472Z Author       : Microsoft
---
2020-03-12T00:25:39.2958622Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-12T00:25:39.2964328Z ##[command]git config gc.auto 0
2020-03-12T00:25:39.2974279Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-12T00:25:39.2978071Z ##[command]git config --get-all http.proxy
2020-03-12T00:25:39.2985286Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69203/merge:refs/remotes/pull/69203/merge
