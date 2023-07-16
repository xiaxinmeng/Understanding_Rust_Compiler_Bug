plain
2019-10-16T22:16:15.2442683Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T22:16:15.2620114Z ##[command]git config gc.auto 0
2019-10-16T22:16:15.2713738Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T22:16:15.2768552Z ##[command]git config --get-all http.proxy
2019-10-16T22:16:15.2907767Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65315/merge:refs/remotes/pull/65315/merge
---
2019-10-16T22:23:18.3727379Z    Compiling serde_json v1.0.40
2019-10-16T22:23:20.2903669Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-16T22:23:32.4157338Z     Finished release [optimized] target(s) in 1m 32s
2019-10-16T22:23:32.4252934Z tidy check
2019-10-16T22:23:32.8921479Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:135: line longer than 100 chars
2019-10-16T22:23:32.8921617Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:213: line longer than 100 chars
2019-10-16T22:23:32.8921710Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:217: line longer than 100 chars
2019-10-16T22:23:32.8948052Z tidy error: /checkout/src/librustc_mir/transform/check_consts/validation.rs:397: line longer than 100 chars
2019-10-16T22:23:32.8973804Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:52: line longer than 100 chars
2019-10-16T22:23:32.8973916Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:128: line longer than 100 chars
2019-10-16T22:23:33.0912215Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3007) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-16T22:23:33.0989634Z tidy error: /checkout/src/librustc/mir/mod.rs:1873: line longer than 100 chars
2019-10-16T22:23:34.6704671Z some tidy checks failed
2019-10-16T22:23:34.6706002Z Found 482 error codes
2019-10-16T22:23:34.6706174Z Found 0 error codes with no tests
2019-10-16T22:23:34.6706617Z Done!
2019-10-16T22:23:34.6706617Z Done!
2019-10-16T22:23:34.6706713Z 
2019-10-16T22:23:34.6706767Z 
2019-10-16T22:23:34.6707907Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-16T22:23:34.6708585Z 
2019-10-16T22:23:34.6708642Z 
2019-10-16T22:23:34.6728807Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-16T22:23:34.6729035Z Build completed unsuccessfully in 0:01:35
2019-10-16T22:23:34.6729035Z Build completed unsuccessfully in 0:01:35
2019-10-16T22:23:34.6785000Z == clock drift check ==
2019-10-16T22:23:34.6803511Z   local time: Wed Oct 16 22:23:34 UTC 2019
2019-10-16T22:23:34.7768468Z   network time: Wed, 16 Oct 2019 22:23:34 GMT
2019-10-16T22:23:34.7769588Z == end clock drift check ==
2019-10-16T22:23:36.1832588Z ##[error]Bash exited with code '1'.
2019-10-16T22:23:36.1868078Z ##[section]Starting: Checkout
2019-10-16T22:23:36.1870375Z ==============================================================================
2019-10-16T22:23:36.1870439Z Task         : Get sources
2019-10-16T22:23:36.1870491Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
