plain
2019-07-20T16:12:53.6559897Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T16:12:53.6788025Z ##[command]git config gc.auto 0
2019-07-20T16:12:53.6859827Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T16:12:53.6906260Z ##[command]git config --get-all http.proxy
2019-07-20T16:12:53.7030778Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-20T16:13:24.9093322Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T16:13:24.9093474Z 
2019-07-20T16:13:24.9093777Z   git checkout -b <new-branch-name>
2019-07-20T16:13:24.9093912Z 
2019-07-20T16:13:24.9094068Z HEAD is now at 245dac9b5 Merge 3eab1234db9deff920de2830e2240be842c7f91f into 95b1fe560d2bd8472f250fb8cfd2168520a58405
2019-07-20T16:13:24.9236962Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T16:13:24.9239878Z ==============================================================================
2019-07-20T16:13:24.9240116Z Task         : Bash
2019-07-20T16:13:24.9240163Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T16:16:35.7890760Z #############################                                             40.5%
2019-07-20T16:16:35.7894488Z ######################################################################## 100.0%
2019-07-20T16:16:36.4465705Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-20T16:16:37.4434504Z     Updating crates.io index
2019-07-20T16:16:57.0928428Z     Updating git repository `https://github.com/crlf0710/libtest`
2019-07-20T16:16:57.7676936Z     Updating git repository `https://github.com/crlf0710/termcolor`
2019-07-20T16:16:58.6131484Z error: failed to write /checkout/Cargo.lock
2019-07-20T16:16:58.6131896Z Caused by:
2019-07-20T16:16:58.6132213Z   failed to open: /checkout/Cargo.lock
2019-07-20T16:16:58.6132243Z 
2019-07-20T16:16:58.6132469Z Caused by:
2019-07-20T16:16:58.6132469Z Caused by:
2019-07-20T16:16:58.6133156Z   Read-only file system (os error 30)
2019-07-20T16:16:58.6201703Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-07-20T16:16:58.6201832Z Build completed unsuccessfully in 0:00:47
2019-07-20T16:17:06.1238343Z ##[error]Bash exited with code '1'.
2019-07-20T16:17:06.1267974Z ##[section]Starting: Checkout
2019-07-20T16:17:06.1269545Z ==============================================================================
2019-07-20T16:17:06.1269589Z Task         : Get sources
2019-07-20T16:17:06.1269625Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
