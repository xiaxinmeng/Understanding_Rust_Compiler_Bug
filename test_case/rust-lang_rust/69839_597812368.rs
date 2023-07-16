plain
2020-03-11T18:45:12.8502810Z ========================== Starting Command Output ===========================
2020-03-11T18:45:12.8507140Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c2961dc3-496e-4ce9-a2c4-edc50521887e.sh
2020-03-11T18:45:12.8507525Z 
2020-03-11T18:45:12.8511905Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T18:45:12.8534576Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69839/merge to s
2020-03-11T18:45:12.8538862Z Task         : Get sources
2020-03-11T18:45:12.8539294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T18:45:12.8539548Z Version      : 1.0.0
2020-03-11T18:45:12.8539766Z Author       : Microsoft
---
2020-03-11T18:45:15.6022307Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T18:45:15.6032570Z ##[command]git config gc.auto 0
2020-03-11T18:45:15.6036048Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T18:45:15.6039262Z ##[command]git config --get-all http.proxy
2020-03-11T18:45:15.6046724Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69839/merge:refs/remotes/pull/69839/merge
