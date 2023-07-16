plain
2020-01-16T20:37:35.9563445Z ========================== Starting Command Output ===========================
2020-01-16T20:37:35.9565010Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fe1d4e29-afd8-4a4e-bc4f-f5e8fa2bb6bf.sh
2020-01-16T20:37:35.9565045Z 
2020-01-16T20:37:35.9567914Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-16T20:37:35.9574433Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68093/merge to s
2020-01-16T20:37:35.9576263Z Task         : Get sources
2020-01-16T20:37:35.9576295Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-16T20:37:35.9576378Z Version      : 1.0.0
2020-01-16T20:37:35.9576410Z Author       : Microsoft
---
2020-01-16T20:37:37.0180309Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T20:37:37.0195196Z ##[command]git config gc.auto 0
2020-01-16T20:37:37.0197842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T20:37:37.0200228Z ##[command]git config --get-all http.proxy
2020-01-16T20:37:37.0207601Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68093/merge:refs/remotes/pull/68093/merge
