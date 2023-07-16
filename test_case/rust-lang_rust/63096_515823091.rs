plain
2019-07-29T02:11:51.4902654Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T02:11:51.5091722Z ##[command]git config gc.auto 0
2019-07-29T02:11:51.5165821Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T02:11:51.5224468Z ##[command]git config --get-all http.proxy
2019-07-29T02:11:51.5348546Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63096/merge:refs/remotes/pull/63096/merge
---
2019-07-29T02:12:27.2663484Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T02:12:27.2663516Z 
2019-07-29T02:12:27.2663730Z   git checkout -b <new-branch-name>
2019-07-29T02:12:27.2663762Z 
2019-07-29T02:12:27.2663828Z HEAD is now at a598c458b Merge 0b8a0e96670596585d96a8645fd7e984dee4ce73 into c7312fe4ff85ada30103cea58db25d83e0bec4b0
2019-07-29T02:12:27.2809604Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T02:12:27.2812802Z ==============================================================================
2019-07-29T02:12:27.2812862Z Task         : Bash
2019-07-29T02:12:27.2812924Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T02:18:42.4741358Z    Compiling serde_json v1.0.40
2019-07-29T02:18:46.8807840Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-29T02:18:55.6787328Z     Finished release [optimized] target(s) in 1m 31s
2019-07-29T02:18:55.6867497Z tidy check
2019-07-29T02:18:56.0816401Z tidy error: /checkout/src/test/ui/existential_types/issue-60564.rs: too many trailing newlines (2)
2019-07-29T02:18:57.5669465Z some tidy checks failed
2019-07-29T02:18:57.5669681Z 
2019-07-29T02:18:57.5669681Z 
2019-07-29T02:18:57.5670556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-29T02:18:57.5670669Z 
2019-07-29T02:18:57.5670729Z 
2019-07-29T02:18:57.5683868Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-29T02:18:57.5683951Z Build completed unsuccessfully in 0:01:34
2019-07-29T02:18:57.5683951Z Build completed unsuccessfully in 0:01:34
2019-07-29T02:18:58.8540864Z ##[error]Bash exited with code '1'.
2019-07-29T02:18:58.8578891Z ##[section]Starting: Checkout
2019-07-29T02:18:58.8580662Z ==============================================================================
2019-07-29T02:18:58.8580739Z Task         : Get sources
2019-07-29T02:18:58.8580783Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
