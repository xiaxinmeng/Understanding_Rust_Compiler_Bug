plain
2019-08-22T22:30:32.8822202Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T22:30:32.9014760Z ##[command]git config gc.auto 0
2019-08-22T22:30:32.9086774Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T22:30:32.9135315Z ##[command]git config --get-all http.proxy
2019-08-22T22:30:32.9300308Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-22T22:31:05.5933079Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T22:31:05.5933131Z 
2019-08-22T22:31:05.5933386Z   git checkout -b <new-branch-name>
2019-08-22T22:31:05.5933419Z 
2019-08-22T22:31:05.5933472Z HEAD is now at 456c55698 Merge a1a707019ba4254f865225ae37d33170b4fa8a92 into 760226733e940cb375f791e894fbb554555eeb01
2019-08-22T22:31:05.6087244Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T22:31:05.6089724Z ==============================================================================
2019-08-22T22:31:05.6089773Z Task         : Bash
2019-08-22T22:31:05.6089813Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T22:37:44.0973699Z    Compiling serde_json v1.0.40
2019-08-22T22:37:46.0723474Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-22T22:37:57.3928515Z     Finished release [optimized] target(s) in 1m 34s
2019-08-22T22:37:57.4017566Z tidy check
2019-08-22T22:37:57.8470145Z tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:114: line longer than 100 chars
2019-08-22T22:37:59.4299769Z some tidy checks failed
2019-08-22T22:37:59.4307972Z 
2019-08-22T22:37:59.4307972Z 
2019-08-22T22:37:59.4309273Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-22T22:37:59.4309408Z 
2019-08-22T22:37:59.4309439Z 
2019-08-22T22:37:59.4317987Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-22T22:37:59.4318457Z Build completed unsuccessfully in 0:01:37
2019-08-22T22:37:59.4318457Z Build completed unsuccessfully in 0:01:37
2019-08-22T22:37:59.4367711Z == clock drift check ==
2019-08-22T22:37:59.4381967Z   local time: Thu Aug 22 22:37:59 UTC 2019
2019-08-22T22:37:59.7158672Z   network time: Thu, 22 Aug 2019 22:37:59 GMT
2019-08-22T22:37:59.7163123Z == end clock drift check ==
2019-08-22T22:38:01.0483862Z ##[error]Bash exited with code '1'.
2019-08-22T22:38:01.0517828Z ##[section]Starting: Checkout
2019-08-22T22:38:01.0520119Z ==============================================================================
2019-08-22T22:38:01.0520184Z Task         : Get sources
2019-08-22T22:38:01.0520261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
