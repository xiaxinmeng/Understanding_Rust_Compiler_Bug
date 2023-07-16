plain
2019-11-01T19:18:29.1878719Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-01T19:18:29.2154607Z ##[command]git config gc.auto 0
2019-11-01T19:18:29.2278773Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-01T19:18:29.2343358Z ##[command]git config --get-all http.proxy
2019-11-01T19:18:29.7677154Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66022/merge:refs/remotes/pull/66022/merge
---
2019-11-01T19:27:38.0534552Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-11-01T19:27:38.8034383Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-11-01T19:27:44.3704194Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-11-01T19:27:48.4713171Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2019-11-01T19:27:49.2081159Z error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `boxed::Box<[_]>`:
2019-11-01T19:27:49.2100973Z    --> src/liballoc/boxed.rs:938:1
2019-11-01T19:27:49.2102993Z 938 | impl<T> IntoIterator for Box<[T]> {
2019-11-01T19:27:49.2103766Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-01T19:27:49.2104485Z     |
2019-11-01T19:27:49.2105518Z     = note: conflicting implementation in crate `core`:
2019-11-01T19:27:49.2105518Z     = note: conflicting implementation in crate `core`:
2019-11-01T19:27:49.2107829Z             - impl<I> core::iter::IntoIterator for I
2019-11-01T19:27:49.2110127Z               where I: core::iter::Iterator;
2019-11-01T19:27:49.2110748Z     = note: upstream crates may add a new impl of trait `core::iter::Iterator` for type `[_]` in future versions
2019-11-01T19:27:49.2110921Z 
2019-11-01T19:27:49.2230220Z error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `&boxed::Box<_>`:
2019-11-01T19:27:49.2231080Z    --> src/liballoc/boxed.rs:947:1
2019-11-01T19:27:49.2231533Z     |
2019-11-01T19:27:49.2232030Z 947 | / impl<'a, I> IntoIterator for &'a Box<I>
2019-11-01T19:27:49.2232491Z 948 | | where
2019-11-01T19:27:49.2233431Z 949 | |     &'a I: IntoIterator,
2019-11-01T19:27:49.2234375Z 950 | | {
2019-11-01T19:27:49.2235184Z 955 | |     }
2019-11-01T19:27:49.2236576Z 956 | | }
2019-11-01T19:27:49.2237036Z     | |_^
2019-11-01T19:27:49.2239037Z     |
2019-11-01T19:27:49.2239037Z     |
2019-11-01T19:27:49.2239586Z     = note: conflicting implementation in crate `core`:
2019-11-01T19:27:49.2240036Z             - impl<I> core::iter::IntoIterator for I
2019-11-01T19:27:49.2240445Z               where I: core::iter::Iterator;
2019-11-01T19:27:49.2241188Z     = note: downstream crates may implement trait `core::iter::Iterator` for type `&boxed::Box<_>`
2019-11-01T19:27:49.2241748Z     = note: downstream crates may implement trait `core::iter::Iterator` for type `&_`
2019-11-01T19:27:49.2241915Z 
2019-11-01T19:27:49.2242390Z error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `&mut boxed::Box<_>`:
2019-11-01T19:27:49.2242814Z    --> src/liballoc/boxed.rs:959:1
2019-11-01T19:27:49.2243207Z     |
2019-11-01T19:27:49.2244010Z 959 | / impl<'a, I> IntoIterator for &'a mut Box<I>
2019-11-01T19:27:49.2244559Z 960 | | where
2019-11-01T19:27:49.2250383Z 961 | |     &'a mut I: IntoIterator
2019-11-01T19:27:49.2253239Z 962 | | {
2019-11-01T19:27:49.2301642Z 967 | |     }
2019-11-01T19:27:49.2302007Z 968 | | }
2019-11-01T19:27:49.2302248Z     | |_^
2019-11-01T19:27:49.2302466Z     |
2019-11-01T19:27:49.2302466Z     |
2019-11-01T19:27:49.2302765Z     = note: conflicting implementation in crate `core`:
2019-11-01T19:27:49.2302999Z             - impl<I> core::iter::IntoIterator for I
2019-11-01T19:27:49.2303243Z               where I: core::iter::Iterator;
2019-11-01T19:27:49.2367361Z error: aborting due to 3 previous errors
2019-11-01T19:27:49.2367435Z 
2019-11-01T19:27:49.2370343Z For more information about this error, try `rustc --explain E0119`.
2019-11-01T19:27:49.2546179Z error: could not compile `alloc`.
---
2019-11-01T19:27:49.6603020Z   local time: Fri Nov  1 19:27:49 UTC 2019
2019-11-01T19:27:49.7128048Z   network time: Fri, 01 Nov 2019 19:27:49 GMT
2019-11-01T19:27:49.7135770Z == end clock drift check ==
2019-11-01T19:27:52.3617975Z 
2019-11-01T19:27:52.3767322Z ##[error]Bash exited with code '1'.
2019-11-01T19:27:52.3801482Z ##[section]Starting: Checkout
2019-11-01T19:27:52.3803719Z ==============================================================================
2019-11-01T19:27:52.3803794Z Task         : Get sources
2019-11-01T19:27:52.3803843Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
