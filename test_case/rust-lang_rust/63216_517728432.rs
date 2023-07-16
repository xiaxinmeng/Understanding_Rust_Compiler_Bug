plain
2019-08-02T14:29:42.2977970Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T14:29:42.3162418Z ##[command]git config gc.auto 0
2019-08-02T14:29:42.3235881Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T14:29:42.3284672Z ##[command]git config --get-all http.proxy
2019-08-02T14:29:42.3420054Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63216/merge:refs/remotes/pull/63216/merge
---
2019-08-02T14:30:17.4953393Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T14:30:17.4953419Z 
2019-08-02T14:30:17.4953575Z   git checkout -b <new-branch-name>
2019-08-02T14:30:17.4953598Z 
2019-08-02T14:30:17.4953651Z HEAD is now at 214f52274 Merge bf4e73ffe4365cb29041095f0e41c3f9d6fcb20e into fc3ef9698fa80aa2f4da6208b8295bc8fa48eec5
2019-08-02T14:30:17.5096840Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T14:30:17.5099753Z ==============================================================================
2019-08-02T14:30:17.5099797Z Task         : Bash
2019-08-02T14:30:17.5099847Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T14:36:06.9038673Z    Compiling serde_json v1.0.40
2019-08-02T14:36:10.8165911Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-02T14:36:18.5465145Z     Finished release [optimized] target(s) in 1m 20s
2019-08-02T14:36:18.5529298Z tidy check
2019-08-02T14:36:19.0453311Z tidy error: /checkout/src/libstd/io/mod.rs:359: line longer than 100 chars
2019-08-02T14:36:20.3035740Z some tidy checks failed
2019-08-02T14:36:20.3040389Z 
2019-08-02T14:36:20.3040389Z 
2019-08-02T14:36:20.3041694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T14:36:20.3042040Z 
2019-08-02T14:36:20.3042062Z 
2019-08-02T14:36:20.3046374Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T14:36:20.3046499Z Build completed unsuccessfully in 0:01:23
2019-08-02T14:36:20.3046499Z Build completed unsuccessfully in 0:01:23
2019-08-02T14:36:21.7480562Z ##[error]Bash exited with code '1'.
2019-08-02T14:36:21.7508993Z ##[section]Starting: Checkout
2019-08-02T14:36:21.7510348Z ==============================================================================
2019-08-02T14:36:21.7510390Z Task         : Get sources
2019-08-02T14:36:21.7510426Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
