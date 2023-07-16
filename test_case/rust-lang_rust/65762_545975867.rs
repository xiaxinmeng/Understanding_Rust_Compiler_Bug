plain
2019-10-24T15:35:21.7153602Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:35:21.7401966Z ##[command]git config gc.auto 0
2019-10-24T15:35:21.7478661Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:35:21.7536464Z ##[command]git config --get-all http.proxy
2019-10-24T15:35:21.7684512Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65762/merge:refs/remotes/pull/65762/merge
---
2019-10-24T15:36:00.1089065Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/daef462e-aaae-4d8a-9de5-bb5ec652fc66.sh
2019-10-24T15:36:00.1435249Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-10-24T15:36:00.1436928Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:36:00.1437447Z 
2019-10-24T15:36:00.2898357Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:36:00.2966398Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:36:00.2966482Z 
2019-10-24T15:36:00.2966482Z 
2019-10-24T15:36:00.3202483Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:36:00.3239784Z 
2019-10-24T15:36:00.3361870Z ##[error]Bash exited with code '7'.
2019-10-24T15:36:00.3503258Z ##[section]Starting: Checkout
2019-10-24T15:36:00.3505062Z ==============================================================================
2019-10-24T15:36:00.3505163Z Task         : Get sources
2019-10-24T15:36:00.3505204Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
