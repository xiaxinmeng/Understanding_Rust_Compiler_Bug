plain
2019-08-09T06:19:28.1718588Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T06:19:28.1923432Z ##[command]git config gc.auto 0
2019-08-09T06:19:28.1992606Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T06:19:28.2053340Z ##[command]git config --get-all http.proxy
2019-08-09T06:19:28.2211548Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63402/merge:refs/remotes/pull/63402/merge
---
2019-08-09T06:20:03.5115857Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T06:20:03.5115913Z 
2019-08-09T06:20:03.5116178Z   git checkout -b <new-branch-name>
2019-08-09T06:20:03.5116214Z 
2019-08-09T06:20:03.5116280Z HEAD is now at 9ac753be9 Merge 5873a187f559f18eb84dfe6dc06119b0ab9bff77 into 5aa3d9a7b5d3a46a7f158e8881146331a6bc9243
2019-08-09T06:20:03.5276613Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T06:20:03.5279885Z ==============================================================================
2019-08-09T06:20:03.5279992Z Task         : Bash
2019-08-09T06:20:03.5280045Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T06:26:18.2232007Z    Compiling serde_json v1.0.40
2019-08-09T06:26:22.7806528Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-09T06:26:31.9661996Z     Finished release [optimized] target(s) in 1m 41s
2019-08-09T06:26:31.9735066Z tidy check
2019-08-09T06:26:32.4693273Z tidy error: /checkout/src/librustc_errors/emitter.rs:260: line longer than 100 chars
2019-08-09T06:26:33.9864636Z Stray file with UI testing output: "/checkout/src/test/ui/feature-gates/feature-gate-existential-type.stderr"
2019-08-09T06:26:33.9897010Z Stray file with UI testing output: "/checkout/src/test/ui/feature-gate/await-macro.stderr"
2019-08-09T06:26:34.0167286Z Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-deref.stderr"
2019-08-09T06:26:34.0169306Z Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/option-deref.stderr"
2019-08-09T06:26:34.0170032Z Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-deref-ok.stderr"
2019-08-09T06:26:34.0174872Z Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-deref-err.stderr"
2019-08-09T06:26:34.0218175Z Stray file with UI testing output: "/checkout/src/test/ui/issues/issue-26158.stderr"
2019-08-09T06:26:34.0270542Z Stray file with UI testing output: "/checkout/src/test/ui/allocator-submodule.stderr"
2019-08-09T06:26:34.0283340Z some tidy checks failed
2019-08-09T06:26:34.0290351Z 
2019-08-09T06:26:34.0290351Z 
2019-08-09T06:26:34.0291616Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-09T06:26:34.0291803Z 
2019-08-09T06:26:34.0291828Z 
2019-08-09T06:26:34.0294396Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-09T06:26:34.0294461Z Build completed unsuccessfully in 0:01:44
2019-08-09T06:26:34.0294461Z Build completed unsuccessfully in 0:01:44
2019-08-09T06:26:35.7435023Z ##[error]Bash exited with code '1'.
2019-08-09T06:26:35.7465993Z ##[section]Starting: Checkout
2019-08-09T06:26:35.7467639Z ==============================================================================
2019-08-09T06:26:35.7467696Z Task         : Get sources
2019-08-09T06:26:35.7467761Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
