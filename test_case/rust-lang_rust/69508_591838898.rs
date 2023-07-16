plain
2020-02-27T08:09:49.4724289Z ========================== Starting Command Output ===========================
2020-02-27T08:09:49.4743760Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/87370e11-e72a-45d2-98bd-85012b571184.sh
2020-02-27T08:09:49.7436971Z 
2020-02-27T08:09:49.7507789Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-27T08:09:49.7544768Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69508/merge to s
2020-02-27T08:09:49.7561117Z Task         : Get sources
2020-02-27T08:09:49.7561604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T08:09:49.7562314Z Version      : 1.0.0
2020-02-27T08:09:49.7562681Z Author       : Microsoft
---
2020-02-27T08:09:52.4720938Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-27T08:09:52.4901189Z ##[command]git config gc.auto 0
2020-02-27T08:09:52.4940899Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-27T08:09:52.4971996Z ##[command]git config --get-all http.proxy
2020-02-27T08:09:52.5065167Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69508/merge:refs/remotes/pull/69508/merge
