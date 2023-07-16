plain
2020-03-02T20:07:46.2401610Z ========================== Starting Command Output ===========================
2020-03-02T20:07:46.2405761Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/941d5007-d83f-4014-a60a-2451b3e7563d.sh
2020-03-02T20:07:46.2406220Z 
2020-03-02T20:07:46.2410843Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T20:07:46.2433527Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69645/merge to s
2020-03-02T20:07:46.2439348Z Task         : Get sources
2020-03-02T20:07:46.2439642Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T20:07:46.2440077Z Version      : 1.0.0
2020-03-02T20:07:46.2440291Z Author       : Microsoft
---
2020-03-02T20:07:47.2294020Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T20:07:47.2299528Z ##[command]git config gc.auto 0
2020-03-02T20:07:47.2303221Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T20:07:47.2313725Z ##[command]git config --get-all http.proxy
2020-03-02T20:07:47.2320028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69645/merge:refs/remotes/pull/69645/merge
