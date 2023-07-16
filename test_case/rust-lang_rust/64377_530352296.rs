plain
2019-09-11T11:52:08.6257281Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T11:52:08.6451088Z ##[command]git config gc.auto 0
2019-09-11T11:52:08.6568992Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T11:52:08.6680692Z ##[command]git config --get-all http.proxy
2019-09-11T11:52:08.6910536Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64377/merge:refs/remotes/pull/64377/merge
---
2019-09-11T12:00:15.8724380Z    Compiling serde_json v1.0.40
2019-09-11T12:00:18.5051790Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-11T12:00:32.4324508Z     Finished release [optimized] target(s) in 1m 54s
2019-09-11T12:00:32.4438674Z tidy check
2019-09-11T12:00:32.9426216Z tidy error: /checkout/src/librustc_mir/error_codes.rs:1178: unexplained "