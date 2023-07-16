plain
2019-08-07T14:49:41.5497784Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T14:49:41.5668159Z ##[command]git config gc.auto 0
2019-08-07T14:49:41.5738370Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T14:49:41.5788871Z ##[command]git config --get-all http.proxy
2019-08-07T14:49:41.5918361Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63350/merge:refs/remotes/pull/63350/merge
---
2019-08-07T14:50:18.1774312Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T14:50:18.1774367Z 
2019-08-07T14:50:18.1774578Z   git checkout -b <new-branch-name>
2019-08-07T14:50:18.1774628Z 
2019-08-07T14:50:18.1774674Z HEAD is now at ee41ceb39 Merge 8f5ceaaa466e45e282207527730bd9be658caeb3 into d4abb08be6c3a06a14e285396f5e3ef367584f77
2019-08-07T14:50:18.1934291Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T14:50:18.1936655Z ==============================================================================
2019-08-07T14:50:18.1936703Z Task         : Bash
2019-08-07T14:50:18.1936744Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T14:55:09.4294038Z    |
2019-08-07T14:55:09.4294517Z 76 | #![feature(const_generics)]
2019-08-07T14:55:09.4294917Z    |            ^^^^^^^^^^^^^^
2019-08-07T14:55:09.4295073Z 
2019-08-07T14:55:12.4747563Z error[E0221]: ambiguous associated type `Item` in bounds of `I`
2019-08-07T14:55:12.4747956Z    --> src/libcore/iter/adapters/flatten.rs:112:30
2019-08-07T14:55:12.4748190Z     |
2019-08-07T14:55:12.4748565Z 112 |     inner: FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>,
2019-08-07T14:55:12.4748946Z     |                              ^^^^^^^ ambiguous associated type `Item`
2019-08-07T14:55:12.4749478Z    ::: src/libcore/iter/traits/collect.rs:211:5
2019-08-07T14:55:12.4749700Z     |
2019-08-07T14:55:12.4750149Z 211 |     type Item;
2019-08-07T14:55:12.4750149Z 211 |     type Item;
2019-08-07T14:55:12.4751000Z     |     ---------- ambiguous `Item` from `iter::traits::collect::IntoIterator`
2019-08-07T14:55:12.4751453Z    ::: src/libcore/iter/traits/iterator.rs:94:5
2019-08-07T14:55:12.4751856Z     |
2019-08-07T14:55:12.4752217Z 94  |     type Item;
2019-08-07T14:55:12.4752217Z 94  |     type Item;
2019-08-07T14:55:12.4752555Z     |     ---------- ambiguous `Item` from `iter::traits::iterator::Iterator`
2019-08-07T14:55:12.4760333Z 
2019-08-07T14:55:12.6160426Z error[E0221]: ambiguous associated type `Item` in bounds of `U`
2019-08-07T14:55:12.6161673Z    --> src/libcore/iter/adapters/flatten.rs:77:39
2019-08-07T14:55:12.6162288Z     |
2019-08-07T14:55:12.6162987Z 77  |     fn next_back(&mut self) -> Option<U::Item> { self.inner.next_back() }
2019-08-07T14:55:12.6164449Z     |                                       ^^^^^^^ ambiguous associated type `Item`
2019-08-07T14:55:12.6165333Z    ::: src/libcore/iter/traits/iterator.rs:94:5
2019-08-07T14:55:12.6165688Z     |
2019-08-07T14:55:12.6166651Z 94  |     type Item;
2019-08-07T14:55:12.6166651Z 94  |     type Item;
2019-08-07T14:55:12.6167254Z     |     ---------- ambiguous `Item` from `iter::traits::iterator::Iterator`
2019-08-07T14:55:12.6168085Z    ::: src/libcore/iter/traits/collect.rs:211:5
2019-08-07T14:55:12.6168461Z     |
2019-08-07T14:55:12.6168871Z 211 |     type Item;
2019-08-07T14:55:12.6168871Z 211 |     type Item;
2019-08-07T14:55:12.6169407Z     |     ---------- ambiguous `Item` from `iter::traits::collect::IntoIterator`
2019-08-07T14:55:12.6727389Z error: aborting due to 2 previous errors
2019-08-07T14:55:12.6727525Z 
2019-08-07T14:55:12.6728590Z For more information about this error, try `rustc --explain E0221`.
2019-08-07T14:55:12.7515295Z error: Could not compile `core`.
2019-08-07T14:55:12.7515295Z error: Could not compile `core`.
2019-08-07T14:55:12.7516647Z warning: build failed, waiting for other jobs to finish...
2019-08-07T14:55:15.4951226Z error: build failed
2019-08-07T14:55:15.4974956Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-07T14:55:15.4986846Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-07T14:55:15.4986986Z Build completed unsuccessfully in 0:02:07
2019-08-07T14:55:15.4986986Z Build completed unsuccessfully in 0:02:07
2019-08-07T14:55:28.7629713Z ##[error]Bash exited with code '1'.
2019-08-07T14:55:28.7665609Z ##[section]Starting: Checkout
2019-08-07T14:55:28.7667104Z ==============================================================================
2019-08-07T14:55:28.7667156Z Task         : Get sources
2019-08-07T14:55:28.7667201Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
