plain
2019-10-24T15:31:33.4566466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:31:34.2238605Z ##[command]git config gc.auto 0
2019-10-24T15:31:34.2247984Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:31:34.2250682Z ##[command]git config --get-all http.proxy
2019-10-24T15:31:34.2253936Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65762/merge:refs/remotes/pull/65762/merge
---
2019-10-24T15:32:10.6418730Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-10-24T15:32:10.6419517Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:32:10.6419895Z 
2019-10-24T15:32:10.7641599Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-24T15:32:10.7882288Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:32:10.7959051Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:32:10.7959798Z 
2019-10-24T15:32:10.7959798Z 
2019-10-24T15:32:10.8237584Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:32:10.8281312Z 
2019-10-24T15:32:10.8399663Z ##[error]Bash exited with code '7'.
2019-10-24T15:32:10.8579303Z ##[section]Starting: Checkout
2019-10-24T15:32:10.8581845Z ==============================================================================
2019-10-24T15:32:10.8581914Z Task         : Get sources
2019-10-24T15:32:10.8581969Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
