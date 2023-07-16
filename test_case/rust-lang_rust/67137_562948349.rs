plain
2019-12-08T12:54:45.1197227Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T12:54:45.1397693Z ##[command]git config gc.auto 0
2019-12-08T12:54:45.1470363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T12:54:45.1517047Z ##[command]git config --get-all http.proxy
2019-12-08T12:54:45.1661943Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-08T13:26:33.3003912Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-08T13:26:37.4274249Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-12-08T13:26:38.5226750Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-08T13:26:51.3416924Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-08T13:26:51.8083715Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:51.8084081Z   --> src/libsyntax_pos/lib.rs:75:10
2019-12-08T13:26:51.8084740Z 75 |          RustcDecodable, RustcEncodable, HashStable_Generic)]
2019-12-08T13:26:51.8086619Z    |          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T13:26:51.8086938Z    | 
2019-12-08T13:26:51.8087368Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:51.8087368Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:51.8087776Z    |
2019-12-08T13:26:51.8088024Z 1  | ($ item : item) => { }
2019-12-08T13:26:51.8088491Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:51.8088690Z    |
2019-12-08T13:26:51.8089107Z    = note: for more information, see ***/issues/47809
2019-12-08T13:26:51.8089392Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:51.8089448Z 
2019-12-08T13:26:51.8866456Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:51.8866783Z    --> src/libsyntax_pos/lib.rs:802:39
2019-12-08T13:26:51.8874982Z     |
2019-12-08T13:26:51.8875805Z 802 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
2019-12-08T13:26:51.8876449Z     | 
2019-12-08T13:26:51.8876710Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:51.8877205Z     |
2019-12-08T13:26:51.8877205Z     |
2019-12-08T13:26:51.8879913Z 1   | ($ item : item) => { }
2019-12-08T13:26:51.8880425Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:51.8883690Z     |
2019-12-08T13:26:51.8885046Z     = note: for more information, see ***/issues/47809
2019-12-08T13:26:51.8885946Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:51.8886022Z 
2019-12-08T13:26:52.0877576Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.0878145Z  --> src/libsyntax_pos/edition.rs:9:26
2019-12-08T13:26:52.0878427Z   |
2019-12-08T13:26:52.0878709Z 9 |          RustcEncodable, RustcDecodable, Eq, HashStable_Generic)]
2019-12-08T13:26:52.0879276Z   | 
2019-12-08T13:26:52.0879526Z  ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.0879747Z   |
2019-12-08T13:26:52.0879747Z   |
2019-12-08T13:26:52.0879991Z 1 | ($ item : item) => { }
2019-12-08T13:26:52.0880294Z   | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.0880537Z   |
2019-12-08T13:26:52.0880937Z   = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.0881422Z   = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.0882297Z 
2019-12-08T13:26:52.1003231Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.1003919Z    |
2019-12-08T13:26:52.1004273Z 63 |          RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T13:26:52.1004691Z    |                          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T13:26:52.1004991Z    | 
2019-12-08T13:26:52.1004991Z    | 
2019-12-08T13:26:52.1005312Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.1005581Z    |
2019-12-08T13:26:52.1031493Z 1  | ($ item : item) => { }
2019-12-08T13:26:52.1032392Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.1032694Z    |
2019-12-08T13:26:52.1033137Z    = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.1033515Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.1033592Z 
2019-12-08T13:26:52.1557405Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.1557947Z     |
2019-12-08T13:26:52.1558250Z 689 | #[derive(Clone, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T13:26:52.1558577Z     |                                        ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T13:26:52.1558815Z     | 
2019-12-08T13:26:52.1558815Z     | 
2019-12-08T13:26:52.1559065Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.1559456Z     |
2019-12-08T13:26:52.1559906Z 1   | ($ item : item) => { }
2019-12-08T13:26:52.1560387Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.1560620Z     |
2019-12-08T13:26:52.1561450Z     = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.1561771Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.1561833Z 
2019-12-08T13:26:52.1618363Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.1618898Z     |
2019-12-08T13:26:52.1619167Z 713 | #[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable,
2019-12-08T13:26:52.1619484Z     |                                                      ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T13:26:52.1620036Z     | 
2019-12-08T13:26:52.1620036Z     | 
2019-12-08T13:26:52.1620355Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.1620585Z     |
2019-12-08T13:26:52.1620834Z 1   | ($ item : item) => { }
2019-12-08T13:26:52.1621141Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.1621390Z     |
2019-12-08T13:26:52.1621724Z     = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.1622768Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.1622826Z 
2019-12-08T13:26:52.1686844Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.1687390Z     |
2019-12-08T13:26:52.1687390Z     |
2019-12-08T13:26:52.1687689Z 749 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T13:26:52.1688468Z     | 
2019-12-08T13:26:52.1688726Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.1688962Z     |
2019-12-08T13:26:52.1688962Z     |
2019-12-08T13:26:52.1689375Z 1   | ($ item : item) => { }
2019-12-08T13:26:52.1689691Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.1689928Z     |
2019-12-08T13:26:52.1690262Z     = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.1690583Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.1690776Z 
2019-12-08T13:26:52.1765898Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T13:26:52.1766618Z     |
2019-12-08T13:26:52.1766618Z     |
2019-12-08T13:26:52.1766935Z 767 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T13:26:52.1767524Z     | 
2019-12-08T13:26:52.1767788Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T13:26:52.1768005Z     |
2019-12-08T13:26:52.1768005Z     |
2019-12-08T13:26:52.1768262Z 1   | ($ item : item) => { }
2019-12-08T13:26:52.1768749Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T13:26:52.1768957Z     |
2019-12-08T13:26:52.1769409Z     = note: for more information, see ***/issues/47809
2019-12-08T13:26:52.1769772Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T13:26:52.2557761Z error: aborting due to 8 previous errors
2019-12-08T13:26:52.2590846Z 
2019-12-08T13:26:52.2608725Z For more information about this error, try `rustc --explain E0658`.
2019-12-08T13:26:52.2719615Z error: could not compile `syntax_pos`.
---
2019-12-08T13:26:52.2848730Z   local time: Sun Dec  8 13:26:52 UTC 2019
2019-12-08T13:26:52.4400470Z   network time: Sun, 08 Dec 2019 13:26:52 GMT
2019-12-08T13:26:52.4403593Z == end clock drift check ==
2019-12-08T13:26:53.8484346Z 
2019-12-08T13:26:53.8591550Z ##[error]Bash exited with code '1'.
2019-12-08T13:26:53.8625609Z ##[section]Starting: Checkout
2019-12-08T13:26:53.8627926Z ==============================================================================
2019-12-08T13:26:53.8627993Z Task         : Get sources
2019-12-08T13:26:53.8628034Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
