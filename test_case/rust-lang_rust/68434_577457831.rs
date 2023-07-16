plain
2020-01-23T00:42:09.2818335Z ========================== Starting Command Output ===========================
2020-01-23T00:42:09.2821350Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d3c7e16e-df35-4350-8282-6aee5ef18887.sh
2020-01-23T00:42:09.2821540Z 
2020-01-23T00:42:09.2867194Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T00:42:09.2873297Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68434/merge to s
2020-01-23T00:42:09.2875351Z Task         : Get sources
2020-01-23T00:42:09.2875394Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T00:42:09.2875427Z Version      : 1.0.0
2020-01-23T00:42:09.2875458Z Author       : Microsoft
---
2020-01-23T00:42:10.2199341Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T00:42:10.2298342Z ##[command]git config gc.auto 0
2020-01-23T00:42:10.2370098Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T00:42:10.2423655Z ##[command]git config --get-all http.proxy
2020-01-23T00:42:10.2618275Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68434/merge:refs/remotes/pull/68434/merge
