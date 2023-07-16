plain
2019-12-02T05:55:58.5668438Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T05:55:58.5684816Z ##[command]git config gc.auto 0
2019-12-02T05:55:58.5690766Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T05:55:58.5692760Z ##[command]git config --get-all http.proxy
2019-12-02T05:55:59.7854087Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66947/merge:refs/remotes/pull/66947/merge
---
2019-12-02T07:36:04.9737490Z    Compiling reqwest v0.9.22
2019-12-02T07:36:48.5665121Z    Compiling mdbook-linkcheck v0.5.0
2019-12-02T07:36:59.9884434Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-12-02T07:37:04.8589669Z     Finished release [optimized] target(s) in 11m 16s
2019-12-02T07:38:43.0300402Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/rustc-bug-fix-procedure.html"
2019-12-02T07:38:43.0301983Z     ┌── compiler-team.md:40:3 ───
2019-12-02T07:38:43.0302472Z     │
2019-12-02T07:38:43.0302472Z     │
2019-12-02T07:38:43.0303037Z  40 │   [aim to give warnings first][procedure]).
2019-12-02T07:38:43.0304724Z     │
2019-12-02T07:38:43.0305128Z 
2019-12-02T07:38:43.0305670Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/platform-support.html"
2019-12-02T07:38:43.0305916Z 
2019-12-02T07:38:43.0305916Z 
2019-12-02T07:38:43.0306584Z      ┌── tests/intro.md:116:8 ───
2019-12-02T07:38:43.0307053Z      │
2019-12-02T07:38:43.0308515Z  116 │ Rust's [platform tiers]).
2019-12-02T07:38:43.0309713Z      │
2019-12-02T07:38:43.0309995Z 
2019-12-02T07:38:43.0310584Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/middle/expr_use_visitor/struct.ExprUseVisitor.html"
2019-12-02T07:38:43.0310881Z 
---
2019-12-02T07:38:43.0326099Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/stabilization-guide.html"
2019-12-02T07:38:43.0326242Z 
2019-12-02T07:38:43.0326743Z     ┌── important-links.md:11:1 ───
2019-12-02T07:38:43.0327258Z     │
2019-12-02T07:38:43.0328461Z  11 │ [link](https://forge.rust-lang.org/stabilization-guide.html#updating-documentation)
2019-12-02T07:38:43.0329414Z     │
2019-12-02T07:38:43.0329582Z 
2019-12-02T07:38:43.0330010Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/stabilization-guide.html"
2019-12-02T07:38:43.0330424Z 
2019-12-02T07:38:43.0330424Z 
2019-12-02T07:38:43.0330826Z     ┌── important-links.md:12:1 ───
2019-12-02T07:38:43.0331186Z     │
2019-12-02T07:38:43.0331806Z  12 │ [link](https://forge.rust-lang.org/stabilization-guide.html#documentation-prs)
2019-12-02T07:38:43.0332725Z     │
2019-12-02T07:38:43.0332882Z 
2019-12-02T07:38:43.0333000Z Error: One or more incorrect links
2019-12-02T07:38:43.0356055Z 
---
2019-12-02T07:51:57.8562662Z error: internal compiler error: unexpected panic
2019-12-02T07:51:57.8562686Z 
2019-12-02T07:51:57.8563106Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8563132Z 
2019-12-02T07:51:57.8563546Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-02T07:51:57.8563630Z note: Clippy version: foo
2019-12-02T07:51:57.8563653Z 
2019-12-02T07:51:57.8567194Z 
2019-12-02T07:51:57.8567263Z 
2019-12-02T07:51:57.8567263Z 
2019-12-02T07:51:57.8570876Z expected stderr:
2019-12-02T07:51:57.8576440Z thread 'rustc' panicked at 'Testing the ICE message', clippy_lints/src/utils/internal_lints.rs
2019-12-02T07:51:57.8576593Z 
2019-12-02T07:51:57.8576638Z error: internal compiler error: unexpected panic
2019-12-02T07:51:57.8576668Z 
2019-12-02T07:51:57.8576713Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8576713Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8576939Z 
2019-12-02T07:51:57.8577368Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-02T07:51:57.8577516Z note: Clippy version: foo
2019-12-02T07:51:57.8577548Z 
2019-12-02T07:51:57.8581884Z 
2019-12-02T07:51:57.8582008Z 
2019-12-02T07:51:57.8582008Z 
2019-12-02T07:51:57.8586054Z diff of stderr:
2019-12-02T07:51:57.8586128Z 
2019-12-02T07:51:57.8590385Z -thread 'rustc' panicked at 'Testing the ICE message', clippy_lints/src/utils/internal_lints.rs
2019-12-02T07:51:57.8611862Z +thread 'rustc' panicked at 'Testing the ICE message', src/tools/clippy/clippy_lints/src/utils/internal_lints.rs
2019-12-02T07:51:57.8612145Z  
2019-12-02T07:51:57.8612201Z  error: internal compiler error: unexpected panic
2019-12-02T07:51:57.8612241Z  
2019-12-02T07:51:57.8612279Z  note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8612279Z  note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8612317Z  
2019-12-02T07:51:57.8612667Z  note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-02T07:51:57.8612750Z  note: Clippy version: foo
2019-12-02T07:51:57.8612810Z  
2019-12-02T07:51:57.8612843Z  
2019-12-02T07:51:57.8612868Z 
2019-12-02T07:51:57.8612868Z 
2019-12-02T07:51:57.8612926Z The actual stderr differed from the expected stderr.
2019-12-02T07:51:57.8613299Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/custom_ice_message.stderr
2019-12-02T07:51:57.8613359Z To update references, run this command from build directory:
2019-12-02T07:51:57.8613737Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base' 'custom_ice_message.rs'
2019-12-02T07:51:57.8613814Z error: 1 errors occurred comparing output.
2019-12-02T07:51:57.8613869Z status: exit code: 101
2019-12-02T07:51:57.8613869Z status: exit code: 101
2019-12-02T07:51:57.8616451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/custom_ice_message.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/custom_ice_message.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4cf2006b2fb0099f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-70d9308013a308bf.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-93610b460d310c52.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/custom_ice_message.stage-id.aux" "-A" "unused"
2019-12-02T07:51:57.8617210Z ------------------------------------------
2019-12-02T07:51:57.8617247Z 
2019-12-02T07:51:57.8617501Z ------------------------------------------
2019-12-02T07:51:57.8617549Z stderr:
---
2019-12-02T07:51:57.8618358Z error: internal compiler error: unexpected panic
2019-12-02T07:51:57.8618407Z 
2019-12-02T07:51:57.8618694Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T07:51:57.8618729Z 
2019-12-02T07:51:57.8619141Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-02T07:51:57.8619180Z 
2019-12-02T07:51:57.8619648Z note: Clippy version: clippy 0.0.212 (45196ce 2019-12-01)
2019-12-02T07:51:57.8619694Z 
2019-12-02T07:51:57.8620053Z ------------------------------------------
2019-12-02T07:51:57.8620076Z 
2019-12-02T07:51:57.8620108Z test [ui] ui/custom_ice_message.rs ... FAILED
---
2019-12-02T08:17:59.6219302Z Verifying status of rustfmt...
2019-12-02T08:17:59.6233201Z Verifying status of clippy-driver...
2019-12-02T08:17:59.6247605Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-02T08:17:59.6261616Z 
2019-12-02T08:17:59.6262681Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-12-02T08:17:59.6263852Z 
2019-12-02T08:17:59.6264498Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-12-02T08:17:59.6264719Z commit another update.
2019-12-02T08:17:59.6264865Z 
2019-12-02T08:17:59.6265311Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-12-02T08:17:59.6266208Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-12-02T08:17:59.6267754Z proper steps.
2019-12-02T08:17:59.6285083Z   local time: Mon Dec  2 08:17:59 UTC 2019
2019-12-02T08:17:59.9192620Z   network time: Mon, 02 Dec 2019 08:17:59 GMT
2019-12-02T08:17:59.9197488Z == end clock drift check ==
2019-12-02T08:18:01.1564512Z 
2019-12-02T08:18:01.1564512Z 
2019-12-02T08:18:01.1668065Z ##[error]Bash exited with code '3'.
2019-12-02T08:18:01.1706354Z ##[section]Starting: Checkout
2019-12-02T08:18:01.1708456Z ==============================================================================
2019-12-02T08:18:01.1708511Z Task         : Get sources
2019-12-02T08:18:01.1708576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
