plain
2020-01-05T12:06:33.8103500Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T12:06:33.8347648Z ##[command]git config gc.auto 0
2020-01-05T12:06:33.8449626Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T12:06:33.8512686Z ##[command]git config --get-all http.proxy
2020-01-05T12:06:33.8687188Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67806/merge:refs/remotes/pull/67806/merge
---
2020-01-05T12:12:44.3811063Z    Compiling serde_json v1.0.40
2020-01-05T12:12:46.2566520Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-05T12:12:56.7983782Z     Finished release [optimized] target(s) in 1m 28s
2020-01-05T12:12:56.8093912Z tidy check
2020-01-05T12:12:57.6927299Z tidy error: /checkout/src/librustc_session/parse.rs:80: unexplained "