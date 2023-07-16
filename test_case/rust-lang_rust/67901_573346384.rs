plain
2020-01-11T19:02:25.1146020Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T19:02:25.1219069Z ##[command]git config gc.auto 0
2020-01-11T19:02:25.1291651Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T19:02:25.1344025Z ##[command]git config --get-all http.proxy
2020-01-11T19:02:25.1482806Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67901/merge:refs/remotes/pull/67901/merge
