plain
2019-10-02T20:17:26.1669931Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-02T20:17:26.1962104Z ##[command]git config gc.auto 0
2019-10-02T20:17:26.2062446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-02T20:17:26.2585518Z ##[command]git config --get-all http.proxy
2019-10-02T20:17:26.2787848Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65020/merge:refs/remotes/pull/65020/merge
---
2019-10-02T20:24:38.4474612Z    Compiling serde_json v1.0.40
2019-10-02T20:24:40.4086005Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-02T20:24:52.4823649Z     Finished release [optimized] target(s) in 1m 36s
2019-10-02T20:24:52.4915599Z tidy check
2019-10-02T20:24:52.8055282Z tidy error: /checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs: too many trailing newlines (2)
2019-10-02T20:24:54.5863050Z some tidy checks failed
2019-10-02T20:24:54.5865905Z 
2019-10-02T20:24:54.5865905Z 
2019-10-02T20:24:54.5867266Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-02T20:24:54.5867953Z 
2019-10-02T20:24:54.5868228Z 
2019-10-02T20:24:54.5873349Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-02T20:24:54.5873690Z Build completed unsuccessfully in 0:01:40
2019-10-02T20:24:54.5873690Z Build completed unsuccessfully in 0:01:40
2019-10-02T20:24:54.5932743Z == clock drift check ==
2019-10-02T20:24:54.5946382Z   local time: Wed Oct  2 20:24:54 UTC 2019
2019-10-02T20:24:54.6663343Z   network time: Wed, 02 Oct 2019 20:24:54 GMT
2019-10-02T20:24:54.6670013Z == end clock drift check ==
2019-10-02T20:24:56.0475069Z ##[error]Bash exited with code '1'.
2019-10-02T20:24:56.0511872Z ##[section]Starting: Checkout
2019-10-02T20:24:56.0514681Z ==============================================================================
2019-10-02T20:24:56.0514760Z Task         : Get sources
2019-10-02T20:24:56.0514809Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
