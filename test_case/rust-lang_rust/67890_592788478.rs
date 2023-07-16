plain
2020-02-29T00:03:15.4686915Z ========================== Starting Command Output ===========================
2020-02-29T00:03:15.4706966Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cda7209e-779c-483b-b775-66f899cc3e5b.sh
2020-02-29T00:03:15.4871434Z 
2020-02-29T00:03:15.4926574Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T00:03:15.4949295Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-02-29T00:03:15.4954124Z Task         : Get sources
2020-02-29T00:03:15.4954592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T00:03:15.4955024Z Version      : 1.0.0
2020-02-29T00:03:15.4955203Z Author       : Microsoft
---
2020-02-29T00:03:16.2888968Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T00:03:16.2941015Z ##[command]git config gc.auto 0
2020-02-29T00:03:16.2973999Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T00:03:16.3004964Z ##[command]git config --get-all http.proxy
2020-02-29T00:03:16.3251082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67890/merge:refs/remotes/pull/67890/merge
