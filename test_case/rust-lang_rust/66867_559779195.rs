plain
2019-11-29T12:31:17.2241088Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T12:31:17.2424814Z ##[command]git config gc.auto 0
2019-11-29T12:31:17.2492202Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T12:31:18.0451047Z ##[command]git config --get-all http.proxy
2019-11-29T12:31:18.0453711Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66867/merge:refs/remotes/pull/66867/merge
---
2019-11-29T12:36:45.4942147Z    Compiling serde_json v1.0.40
2019-11-29T12:36:47.0396857Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-29T12:36:57.1357137Z     Finished release [optimized] target(s) in 1m 20s
2019-11-29T12:36:57.1444482Z tidy check
2019-11-29T12:36:57.8412336Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0036.md:38: unexplained "