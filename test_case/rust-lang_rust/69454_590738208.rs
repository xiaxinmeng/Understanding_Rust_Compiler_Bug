plain
2020-02-25T08:03:19.7743117Z ========================== Starting Command Output ===========================
2020-02-25T08:03:19.7745954Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/accc4588-c2b2-463b-91f5-7aa5bd753620.sh
2020-02-25T08:03:19.7746273Z 
2020-02-25T08:03:19.7750333Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T08:03:19.7774437Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T08:03:19.7779290Z Task         : Get sources
2020-02-25T08:03:19.7779669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T08:03:19.7780018Z Version      : 1.0.0
2020-02-25T08:03:19.7780241Z Author       : Microsoft
---
2020-02-25T08:03:22.3426896Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T08:03:22.3589785Z ##[command]git config gc.auto 0
2020-02-25T08:03:22.3634447Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T08:03:22.3680783Z ##[command]git config --get-all http.proxy
2020-02-25T08:03:22.3794252Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69454/merge:refs/remotes/pull/69454/merge
---
2020-02-25T08:10:40.3534391Z     Checking rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-02-25T08:10:41.8875617Z error[E0277]: the trait bound `T: std::cmp::Ord` is not satisfied
2020-02-25T08:10:41.8877206Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/de/impls.rs:709:21
2020-02-25T08:10:41.8878096Z     |
2020-02-25T08:10:41.8878953Z 690 | / macro_rules! seq_impl {
2020-02-25T08:10:41.8880761Z 691 | |     (
2020-02-25T08:10:41.8882339Z 692 | |         $ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
2020-02-25T08:10:41.8883739Z 693 | |         $access:ident,
2020-02-25T08:10:41.8884717Z ...   |
2020-02-25T08:10:41.8885676Z 708 | |                 struct SeqVisitor<T $(, $typaram)*> {
2020-02-25T08:10:41.8887060Z     | |                                   - help: consider restricting this bound: `T: std::cmp::Ord`
2020-02-25T08:10:41.8888511Z 709 | |                     marker: PhantomData<$ty<T $(, $typaram)*>>,
2020-02-25T08:10:41.8891223Z ...   |
2020-02-25T08:10:41.8892152Z 779 | |     }
2020-02-25T08:10:41.8893798Z 780 | | }
2020-02-25T08:10:41.8893798Z 780 | | }
2020-02-25T08:10:41.8894548Z     | |_- in this expansion of `seq_impl!`
2020-02-25T08:10:41.8895223Z ...
2020-02-25T08:10:41.8895809Z 787 | / seq_impl!(
2020-02-25T08:10:41.8896637Z 788 | |     BinaryHeap<T: Ord>,
2020-02-25T08:10:41.8897463Z 789 | |     seq,
2020-02-25T08:10:41.8898277Z 790 | |     BinaryHeap::clear,
2020-02-25T08:10:41.8899685Z 793 | |     BinaryHeap::push
2020-02-25T08:10:41.8900681Z 794 | | );
2020-02-25T08:10:41.8901439Z     | |__- in this macro invocation
2020-02-25T08:10:41.8903139Z     |
2020-02-25T08:10:41.8903139Z     |
2020-02-25T08:10:41.8907073Z     = note: required because of the requirements on the impl of `for<'r, 's> std::ops::Fn<(&'r T, &'s T)>` for `std::collections::binary_heap::MaxCmp`
2020-02-25T08:10:41.8909004Z 
2020-02-25T08:10:41.9029511Z error[E0277]: the trait bound `T: std::cmp::Ord` is not satisfied
2020-02-25T08:10:41.9030348Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/de/impls.rs:746:72
2020-02-25T08:10:41.9031014Z     |
2020-02-25T08:10:41.9031014Z     |
2020-02-25T08:10:41.9031647Z 690 | / macro_rules! seq_impl {
2020-02-25T08:10:41.9032484Z 691 | |     (
2020-02-25T08:10:41.9033748Z 692 | |         $ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
2020-02-25T08:10:41.9035004Z 693 | |         $access:ident,
2020-02-25T08:10:41.9040906Z ...   |
2020-02-25T08:10:41.9042110Z 746 | |                 struct SeqInPlaceVisitor<'a, T: 'a $(, $typaram: 'a)*>(&'a mut $ty<T $(, $typaram)*>);
2020-02-25T08:10:41.9049804Z     | |                                              |
2020-02-25T08:10:41.9049804Z     | |                                              |
2020-02-25T08:10:41.9051347Z     | |                                              help: consider further restricting this bound: `T: std::cmp::Ord +`
2020-02-25T08:10:41.9053028Z 779 | |     }
2020-02-25T08:10:41.9053723Z 780 | | }
2020-02-25T08:10:41.9053723Z 780 | | }
2020-02-25T08:10:41.9054497Z     | |_- in this expansion of `seq_impl!`
2020-02-25T08:10:41.9055031Z ...
2020-02-25T08:10:41.9055621Z 787 | / seq_impl!(
2020-02-25T08:10:41.9056459Z 788 | |     BinaryHeap<T: Ord>,
2020-02-25T08:10:41.9057261Z 789 | |     seq,
2020-02-25T08:10:41.9058077Z 790 | |     BinaryHeap::clear,
2020-02-25T08:10:41.9059488Z 793 | |     BinaryHeap::push
2020-02-25T08:10:41.9060366Z 794 | | );
2020-02-25T08:10:41.9061206Z     | |__- in this macro invocation
2020-02-25T08:10:41.9061718Z     |
2020-02-25T08:10:41.9061718Z     |
2020-02-25T08:10:41.9062878Z     = note: required because of the requirements on the impl of `for<'r, 's> std::ops::Fn<(&'r T, &'s T)>` for `std::collections::binary_heap::MaxCmp`
2020-02-25T08:10:41.9064453Z 
2020-02-25T08:10:43.0979564Z error: aborting due to 2 previous errors
2020-02-25T08:10:43.0979915Z 
2020-02-25T08:10:43.0987092Z For more information about this error, try `rustc --explain E0277`.
2020-02-25T08:10:43.0987092Z For more information about this error, try `rustc --explain E0277`.
2020-02-25T08:10:43.1026869Z error: could not compile `serde`.
2020-02-25T08:10:43.1027538Z warning: build failed, waiting for other jobs to finish...
2020-02-25T08:10:44.1486644Z error: build failed
2020-02-25T08:10:44.1514543Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-25T08:10:44.1526039Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-25T08:10:44.1526468Z Build completed unsuccessfully in 0:03:33
2020-02-25T08:10:44.1588114Z == clock drift check ==
2020-02-25T08:10:44.1604227Z   local time: Tue Feb 25 08:10:44 UTC 2020
2020-02-25T08:10:44.1604227Z   local time: Tue Feb 25 08:10:44 UTC 2020
2020-02-25T08:10:44.2248733Z   network time: Tue, 25 Feb 2020 08:10:44 GMT
2020-02-25T08:10:44.2252785Z == end clock drift check ==
2020-02-25T08:10:45.4115670Z 
2020-02-25T08:10:45.4203105Z ##[error]Bash exited with code '1'.
2020-02-25T08:10:45.4218283Z ##[section]Finishing: Run build
2020-02-25T08:10:45.4266142Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T08:10:45.4270493Z Task         : Get sources
2020-02-25T08:10:45.4270803Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T08:10:45.4271214Z Version      : 1.0.0
2020-02-25T08:10:45.4271437Z Author       : Microsoft
2020-02-25T08:10:45.4271437Z Author       : Microsoft
2020-02-25T08:10:45.4271760Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T08:10:45.4272136Z ==============================================================================
2020-02-25T08:10:45.7771423Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T08:10:45.7819667Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T08:10:45.7924237Z Cleaning up task key
2020-02-25T08:10:45.7926538Z Start cleaning up orphan processes.
2020-02-25T08:10:45.8150127Z Terminate orphan process: pid (4712) (python)
2020-02-25T08:10:45.8326701Z ##[section]Finishing: Finalize Job
