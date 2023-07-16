plain
2020-02-25T10:54:12.0051904Z ========================== Starting Command Output ===========================
2020-02-25T10:54:12.0054223Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e9e86476-9664-47f1-976f-98fb8a6acf5c.sh
2020-02-25T10:54:12.0054538Z 
2020-02-25T10:54:12.0058554Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T10:54:12.0073612Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T10:54:12.0076998Z Task         : Get sources
2020-02-25T10:54:12.0077241Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T10:54:12.0077465Z Version      : 1.0.0
2020-02-25T10:54:12.0077614Z Author       : Microsoft
---
2020-02-25T10:54:12.9924902Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T10:54:12.9928752Z ##[command]git config gc.auto 0
2020-02-25T10:54:12.9931441Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T10:54:12.9933710Z ##[command]git config --get-all http.proxy
2020-02-25T10:54:12.9938300Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69454/merge:refs/remotes/pull/69454/merge
---
2020-02-25T10:59:17.2457518Z     Checking rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-02-25T10:59:18.1219306Z error[E0277]: the trait bound `T: std::cmp::Ord` is not satisfied
2020-02-25T10:59:18.1225240Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/de/impls.rs:709:21
2020-02-25T10:59:18.1225703Z     |
2020-02-25T10:59:18.1226184Z 690 | / macro_rules! seq_impl {
2020-02-25T10:59:18.1226699Z 691 | |     (
2020-02-25T10:59:18.1227519Z 692 | |         $ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
2020-02-25T10:59:18.1228307Z 693 | |         $access:ident,
2020-02-25T10:59:18.1228775Z ...   |
2020-02-25T10:59:18.1229319Z 708 | |                 struct SeqVisitor<T $(, $typaram)*> {
2020-02-25T10:59:18.1230137Z     | |                                   - help: consider restricting this bound: `T: std::cmp::Ord`
2020-02-25T10:59:18.1231138Z 709 | |                     marker: PhantomData<$ty<T $(, $typaram)*>>,
2020-02-25T10:59:18.1232686Z ...   |
2020-02-25T10:59:18.1233144Z 779 | |     }
2020-02-25T10:59:18.1233627Z 780 | | }
2020-02-25T10:59:18.1233627Z 780 | | }
2020-02-25T10:59:18.1234153Z     | |_- in this expansion of `seq_impl!`
2020-02-25T10:59:18.1234539Z ...
2020-02-25T10:59:18.1234948Z 787 | / seq_impl!(
2020-02-25T10:59:18.1235615Z 788 | |     BinaryHeap<T: Ord>,
2020-02-25T10:59:18.1236172Z 789 | |     seq,
2020-02-25T10:59:18.1236730Z 790 | |     BinaryHeap::clear,
2020-02-25T10:59:18.1237712Z 793 | |     BinaryHeap::push
2020-02-25T10:59:18.1238249Z 794 | | );
2020-02-25T10:59:18.1238777Z     | |__- in this macro invocation
2020-02-25T10:59:18.1239163Z     |
2020-02-25T10:59:18.1239163Z     |
2020-02-25T10:59:18.1239819Z     = note: required because of the requirements on the impl of `for<'r, 's> std::ops::Fn<(&'r T, &'s T)>` for `std::collections::binary_heap::MaxCmp`
2020-02-25T10:59:18.1240759Z 
2020-02-25T10:59:18.1241143Z error[E0277]: the trait bound `T: std::cmp::Ord` is not satisfied
2020-02-25T10:59:18.1241732Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/de/impls.rs:746:72
2020-02-25T10:59:18.1242129Z     |
2020-02-25T10:59:18.1242129Z     |
2020-02-25T10:59:18.1242571Z 690 | / macro_rules! seq_impl {
2020-02-25T10:59:18.1243100Z 691 | |     (
2020-02-25T10:59:18.1243889Z 692 | |         $ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
2020-02-25T10:59:18.1244672Z 693 | |         $access:ident,
2020-02-25T10:59:18.1245136Z ...   |
2020-02-25T10:59:18.1245804Z 746 | |                 struct SeqInPlaceVisitor<'a, T: 'a $(, $typaram: 'a)*>(&'a mut $ty<T $(, $typaram)*>);
2020-02-25T10:59:18.1247839Z     | |                                              |
2020-02-25T10:59:18.1247839Z     | |                                              |
2020-02-25T10:59:18.1248641Z     | |                                              help: consider further restricting this bound: `T: std::cmp::Ord +`
2020-02-25T10:59:18.1249801Z 779 | |     }
2020-02-25T10:59:18.1250285Z 780 | | }
2020-02-25T10:59:18.1250285Z 780 | | }
2020-02-25T10:59:18.1250823Z     | |_- in this expansion of `seq_impl!`
2020-02-25T10:59:18.1251248Z ...
2020-02-25T10:59:18.1251659Z 787 | / seq_impl!(
2020-02-25T10:59:18.1252247Z 788 | |     BinaryHeap<T: Ord>,
2020-02-25T10:59:18.1252802Z 789 | |     seq,
2020-02-25T10:59:18.1253364Z 790 | |     BinaryHeap::clear,
2020-02-25T10:59:18.1254606Z 793 | |     BinaryHeap::push
2020-02-25T10:59:18.1255232Z 794 | | );
2020-02-25T10:59:18.1255841Z     | |__- in this macro invocation
2020-02-25T10:59:18.1256289Z     |
2020-02-25T10:59:18.1256289Z     |
2020-02-25T10:59:18.1257061Z     = note: required because of the requirements on the impl of `for<'r, 's> std::ops::Fn<(&'r T, &'s T)>` for `std::collections::binary_heap::MaxCmp`
2020-02-25T10:59:18.1258128Z 
2020-02-25T10:59:18.9126822Z error: aborting due to 2 previous errors
2020-02-25T10:59:18.9127101Z 
2020-02-25T10:59:18.9127507Z For more information about this error, try `rustc --explain E0277`.
2020-02-25T10:59:18.9127507Z For more information about this error, try `rustc --explain E0277`.
2020-02-25T10:59:18.9157279Z error: could not compile `serde`.
2020-02-25T10:59:18.9157785Z warning: build failed, waiting for other jobs to finish...
2020-02-25T10:59:19.2143660Z error: build failed
2020-02-25T10:59:19.2163510Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-25T10:59:19.2174570Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-25T10:59:19.2174842Z Build completed unsuccessfully in 0:02:44
2020-02-25T10:59:19.2214867Z == clock drift check ==
2020-02-25T10:59:19.2227119Z   local time: Tue Feb 25 10:59:19 UTC 2020
2020-02-25T10:59:19.2227119Z   local time: Tue Feb 25 10:59:19 UTC 2020
2020-02-25T10:59:19.3770467Z   network time: Tue, 25 Feb 2020 10:59:19 GMT
2020-02-25T10:59:19.3774964Z == end clock drift check ==
2020-02-25T10:59:20.7468772Z 
2020-02-25T10:59:20.7519490Z ##[error]Bash exited with code '1'.
2020-02-25T10:59:20.7548072Z ##[section]Finishing: Run build
2020-02-25T10:59:20.7578784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T10:59:20.7582692Z Task         : Get sources
2020-02-25T10:59:20.7582930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T10:59:20.7583162Z Version      : 1.0.0
2020-02-25T10:59:20.7583316Z Author       : Microsoft
2020-02-25T10:59:20.7583316Z Author       : Microsoft
2020-02-25T10:59:20.7583566Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T10:59:20.7583863Z ==============================================================================
2020-02-25T10:59:20.9915298Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T10:59:20.9947993Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69454/merge to s
2020-02-25T10:59:21.0019662Z Cleaning up task key
2020-02-25T10:59:21.0020712Z Start cleaning up orphan processes.
2020-02-25T10:59:21.0149059Z Terminate orphan process: pid (4273) (python)
2020-02-25T10:59:21.0247381Z ##[section]Finishing: Finalize Job
