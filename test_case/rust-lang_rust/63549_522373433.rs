plain
2019-08-19T00:25:46.6478455Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T00:25:46.6677665Z ##[command]git config gc.auto 0
2019-08-19T00:25:46.6764623Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T00:25:46.6818086Z ##[command]git config --get-all http.proxy
2019-08-19T00:25:46.6955846Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63549/merge:refs/remotes/pull/63549/merge
---
2019-08-19T00:26:22.2032755Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T00:26:22.2032785Z 
2019-08-19T00:26:22.2033013Z   git checkout -b <new-branch-name>
2019-08-19T00:26:22.2033043Z 
2019-08-19T00:26:22.2033090Z HEAD is now at 8a1b0e0da Merge 1d2274e31210c689120e1d6ac6194e9b38cc54a2 into 4cf7673076e6975532213e494dd3f7f9d8c2328e
2019-08-19T00:26:22.2214837Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T00:26:22.2217947Z ==============================================================================
2019-08-19T00:26:22.2218027Z Task         : Bash
2019-08-19T00:26:22.2218093Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T00:31:27.7399286Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-19T00:31:33.2496403Z error[E0276]: impl has stricter requirements than trait
2019-08-19T00:31:33.2496762Z     --> src/libcore/iter/adapters/mod.rs:71:5
2019-08-19T00:31:33.2497018Z      |
2019-08-19T00:31:33.2497404Z 71   | /     fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
2019-08-19T00:31:33.2497730Z 72   | |         P: FnMut(Self::Item) -> bool,
2019-08-19T00:31:33.2498103Z 73   | |         P: FnMut(<I as Iterator>::Item) -> bool,
2019-08-19T00:31:33.2498565Z 74   | |         Self: ExactSizeIterator,
2019-08-19T00:31:33.2499092Z 80   | |         })
2019-08-19T00:31:33.2499363Z 81   | |     }
2019-08-19T00:31:33.2499363Z 81   | |     }
2019-08-19T00:31:33.2499739Z      | |_____^ impl has extra requirement `P: ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>`
2019-08-19T00:31:33.2500221Z     ::: src/libcore/iter/traits/iterator.rs:2131:5
2019-08-19T00:31:33.2500447Z      |
2019-08-19T00:31:33.2500447Z      |
2019-08-19T00:31:33.2500766Z 2131 | /     fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
2019-08-19T00:31:33.2501066Z 2132 | |         P: FnMut(Self::Item) -> bool,
2019-08-19T00:31:33.2501407Z 2133 | |         Self: Sized + ExactSizeIterator + DoubleEndedIterator
2019-08-19T00:31:33.2502794Z ...    |
2019-08-19T00:31:33.2502794Z ...    |
2019-08-19T00:31:33.2503178Z 2149 | |         self.try_rfold(n, check(predicate)).break_value()
2019-08-19T00:31:33.2503663Z 2150 | |     }
2019-08-19T00:31:33.2504006Z      | |_____- definition of `rposition` from trait
2019-08-19T00:31:37.0956721Z    Compiling libc v0.2.60
2019-08-19T00:31:38.0326669Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-19T00:31:38.4602899Z error: aborting due to previous error
2019-08-19T00:31:38.4605829Z 
2019-08-19T00:31:38.4605829Z 
2019-08-19T00:31:38.4644742Z For more information about this error, try `rustc --explain E0276`.
2019-08-19T00:31:38.5562933Z error: Could not compile `core`.
2019-08-19T00:31:38.5563341Z warning: build failed, waiting for other jobs to finish...
2019-08-19T00:31:39.1479897Z error: build failed
2019-08-19T00:31:39.1516953Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-19T00:31:39.1523850Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-19T00:31:39.1524112Z Build completed unsuccessfully in 0:02:34
2019-08-19T00:31:39.1579618Z == clock drift check ==
2019-08-19T00:31:39.1591379Z   local time: Mon Aug 19 00:31:39 UTC 2019
2019-08-19T00:31:39.1591379Z   local time: Mon Aug 19 00:31:39 UTC 2019
2019-08-19T00:31:39.3111863Z   network time: Mon, 19 Aug 2019 00:31:39 GMT
2019-08-19T00:31:39.3115286Z == end clock drift check ==
2019-08-19T00:31:49.4092478Z ##[error]Bash exited with code '1'.
2019-08-19T00:31:49.4135707Z ##[section]Starting: Checkout
2019-08-19T00:31:49.4137642Z ==============================================================================
2019-08-19T00:31:49.4137738Z Task         : Get sources
2019-08-19T00:31:49.4137784Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
