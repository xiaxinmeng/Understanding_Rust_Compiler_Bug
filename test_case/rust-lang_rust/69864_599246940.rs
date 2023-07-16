plain
2020-03-15T18:13:12.3750324Z ========================== Starting Command Output ===========================
2020-03-15T18:13:12.3752778Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/413036cd-3fc4-494a-894e-30ad5b6da9c8.sh
2020-03-15T18:13:12.3753053Z 
2020-03-15T18:13:12.3757102Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-15T18:13:12.3775921Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69864/merge to s
2020-03-15T18:13:12.3779226Z Task         : Get sources
2020-03-15T18:13:12.3779537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T18:13:12.3779841Z Version      : 1.0.0
2020-03-15T18:13:12.3780044Z Author       : Microsoft
---
2020-03-15T18:13:13.3857110Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-15T18:13:13.3862383Z ##[command]git config gc.auto 0
2020-03-15T18:13:13.3866202Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-15T18:13:13.3869465Z ##[command]git config --get-all http.proxy
2020-03-15T18:13:13.3875372Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69864/merge:refs/remotes/pull/69864/merge
