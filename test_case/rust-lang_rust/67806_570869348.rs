plain
2020-01-05T05:35:36.4220287Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T05:35:36.4448530Z ##[command]git config gc.auto 0
2020-01-05T05:35:36.4523730Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T05:35:36.4580305Z ##[command]git config --get-all http.proxy
2020-01-05T05:35:36.4731199Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67806/merge:refs/remotes/pull/67806/merge
---
2020-01-05T05:41:52.5861116Z    Compiling serde_json v1.0.40
2020-01-05T05:41:54.2740524Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-05T05:42:04.1925520Z     Finished release [optimized] target(s) in 1m 22s
2020-01-05T05:42:04.2037711Z tidy check
2020-01-05T05:42:05.0761917Z tidy error: /checkout/src/librustc_session/parse.rs:80: unexplained "