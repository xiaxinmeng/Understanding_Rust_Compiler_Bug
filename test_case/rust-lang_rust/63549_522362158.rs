plain
2019-08-18T22:22:47.8176810Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T22:22:47.8392001Z ##[command]git config gc.auto 0
2019-08-18T22:22:47.8470777Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T22:22:47.8517207Z ##[command]git config --get-all http.proxy
2019-08-18T22:22:47.8676645Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63549/merge:refs/remotes/pull/63549/merge
---
2019-08-18T22:23:22.5260822Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T22:23:22.5262380Z 
2019-08-18T22:23:22.5264933Z   git checkout -b <new-branch-name>
2019-08-18T22:23:22.5266170Z 
2019-08-18T22:23:22.5267186Z HEAD is now at 09790f6f2 Merge 137bfd7a7e983267f7ed35607dbed199cb2c6643 into 4cf7673076e6975532213e494dd3f7f9d8c2328e
2019-08-18T22:23:22.5429800Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T22:23:22.5432973Z ==============================================================================
2019-08-18T22:23:22.5433031Z Task         : Bash
2019-08-18T22:23:22.5433097Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T22:28:32.0712231Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-18T22:28:37.5404320Z error[E0276]: impl has stricter requirements than trait
2019-08-18T22:28:37.5405470Z     --> src/libcore/iter/adapters/mod.rs:71:5
2019-08-18T22:28:37.5406729Z      |
2019-08-18T22:28:37.5408158Z 71   | /     fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
2019-08-18T22:28:37.5408980Z 72   | |         P: FnMut(<I as Iterator>::Item) -> bool,
2019-08-18T22:28:37.5409920Z 73   | |         Self: ExactSizeIterator,
2019-08-18T22:28:37.5411156Z ...    |
2019-08-18T22:28:37.5411845Z 79   | |         })
2019-08-18T22:28:37.5412642Z 80   | |     }
2019-08-18T22:28:37.5412642Z 80   | |     }
2019-08-18T22:28:37.5413407Z      | |_____^ impl has extra requirement `P: ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>`
2019-08-18T22:28:37.5414588Z     ::: src/libcore/iter/traits/iterator.rs:2131:5
2019-08-18T22:28:37.5415058Z      |
2019-08-18T22:28:37.5415058Z      |
2019-08-18T22:28:37.5416002Z 2131 | /     fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
2019-08-18T22:28:37.5416798Z 2132 | |         P: FnMut(Self::Item) -> bool,
2019-08-18T22:28:37.5417459Z 2133 | |         Self: Sized + ExactSizeIterator + DoubleEndedIterator
2019-08-18T22:28:37.5418574Z ...    |
2019-08-18T22:28:37.5418574Z ...    |
2019-08-18T22:28:37.5419372Z 2149 | |         self.try_rfold(n, check(predicate)).break_value()
2019-08-18T22:28:37.5419875Z 2150 | |     }
2019-08-18T22:28:37.5420416Z      | |_____- definition of `rposition` from trait
2019-08-18T22:28:41.3679959Z    Compiling libc v0.2.60
2019-08-18T22:28:41.8499858Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-18T22:28:42.5158190Z error: aborting due to previous error
2019-08-18T22:28:42.5158429Z 
2019-08-18T22:28:42.5158429Z 
2019-08-18T22:28:42.5163960Z For more information about this error, try `rustc --explain E0276`.
2019-08-18T22:28:42.6101099Z error: Could not compile `core`.
2019-08-18T22:28:42.6101934Z warning: build failed, waiting for other jobs to finish...
2019-08-18T22:28:43.0729316Z error: build failed
2019-08-18T22:28:43.0752482Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-18T22:28:43.0763120Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-18T22:28:43.0763399Z Build completed unsuccessfully in 0:02:30
2019-08-18T22:28:43.0815158Z == clock drift check ==
2019-08-18T22:28:43.0831648Z   local time: Sun Aug 18 22:28:43 UTC 2019
2019-08-18T22:28:43.0831648Z   local time: Sun Aug 18 22:28:43 UTC 2019
2019-08-18T22:28:43.1673521Z   network time: Sun, 18 Aug 2019 22:28:43 GMT
2019-08-18T22:28:43.1677236Z == end clock drift check ==
2019-08-18T22:28:56.1902649Z ##[error]Bash exited with code '1'.
2019-08-18T22:28:56.1937130Z ##[section]Starting: Checkout
2019-08-18T22:28:56.1938982Z ==============================================================================
2019-08-18T22:28:56.1939038Z Task         : Get sources
2019-08-18T22:28:56.1939088Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
