plain
2019-11-07T12:22:23.8419570Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-07T12:22:23.8606764Z ##[command]git config gc.auto 0
2019-11-07T12:22:23.8687312Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-07T12:22:23.8720475Z ##[command]git config --get-all http.proxy
2019-11-07T12:22:23.8889733Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66156/merge:refs/remotes/pull/66156/merge
---
2019-11-07T12:29:07.3286250Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-07T12:29:07.4751091Z    Compiling backtrace v0.3.40
2019-11-07T12:29:08.0272946Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-11-07T12:29:08.2112142Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-07T12:29:09.9519970Z error[E0658]: the `#[rustc_on_unimplemented]` attribute is an experimental feature
2019-11-07T12:29:09.9521616Z     |
2019-11-07T12:29:09.9522288Z 105 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9522288Z 105 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9523344Z 106 | |     message="the type `{Self}` may not be safely transferred across an unwind boundary",
2019-11-07T12:29:09.9524034Z 107 | |     label="`{Self}` may not be safely transferred across an unwind boundary",
2019-11-07T12:29:09.9524673Z 108 | | )]
2019-11-07T12:29:09.9525314Z     | |__^
2019-11-07T12:29:09.9525916Z     |
2019-11-07T12:29:09.9526700Z     = note: for more information, see ***/issues/29628
2019-11-07T12:29:09.9527230Z     = help: add `#![feature(on_unimplemented)]` to the crate attributes to enable
2019-11-07T12:29:09.9527401Z 
2019-11-07T12:29:09.9527849Z error[E0658]: the `#[rustc_on_unimplemented]` attribute is an experimental feature
2019-11-07T12:29:09.9528591Z     |
2019-11-07T12:29:09.9529035Z 123 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9529035Z 123 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9529593Z 124 | |     message="the type `{Self}` may contain interior mutability and a reference may not be safely \
2019-11-07T12:29:09.9530139Z 125 | |              transferrable across a catch_unwind boundary",
2019-11-07T12:29:09.9530838Z 126 | |     label="`{Self}` may contain interior mutability and a reference may not be safely \
2019-11-07T12:29:09.9531322Z 127 | |            transferrable across a catch_unwind boundary",
2019-11-07T12:29:09.9531762Z 128 | | )]
2019-11-07T12:29:09.9532160Z     | |__^
2019-11-07T12:29:09.9532512Z     |
2019-11-07T12:29:09.9532999Z     = note: for more information, see ***/issues/29628
2019-11-07T12:29:09.9533471Z     = help: add `#![feature(on_unimplemented)]` to the crate attributes to enable
2019-11-07T12:29:09.9533666Z 
2019-11-07T12:29:09.9579890Z error[E0658]: the `#[rustc_on_unimplemented]` attribute is an experimental feature
2019-11-07T12:29:09.9580893Z      |
2019-11-07T12:29:09.9581374Z 1606 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9581374Z 1606 | / #[rustc_on_unimplemented(
2019-11-07T12:29:09.9581881Z 1607 | |   message="`main` has invalid return type `{Self}`",
2019-11-07T12:29:09.9582431Z 1608 | |   label="`main` can only return types that implement `{Termination}`")]
2019-11-07T12:29:09.9583305Z      |
2019-11-07T12:29:09.9583305Z      |
2019-11-07T12:29:09.9583845Z      = note: for more information, see ***/issues/29628
2019-11-07T12:29:09.9584355Z      = help: add `#![feature(on_unimplemented)]` to the crate attributes to enable
2019-11-07T12:29:12.0106781Z error: aborting due to 3 previous errors
2019-11-07T12:29:12.0107665Z 
2019-11-07T12:29:12.0108320Z For more information about this error, try `rustc --explain E0658`.
2019-11-07T12:29:12.0470608Z error: could not compile `std`.
---
2019-11-07T12:29:12.0588829Z   local time: Thu Nov  7 12:29:12 UTC 2019
2019-11-07T12:29:12.2069520Z   network time: Thu, 07 Nov 2019 12:29:12 GMT
2019-11-07T12:29:12.2091825Z == end clock drift check ==
2019-11-07T12:29:15.1993102Z 
2019-11-07T12:29:15.2089323Z ##[error]Bash exited with code '1'.
2019-11-07T12:29:15.2117525Z ##[section]Starting: Checkout
2019-11-07T12:29:15.2119190Z ==============================================================================
2019-11-07T12:29:15.2119409Z Task         : Get sources
2019-11-07T12:29:15.2119459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
