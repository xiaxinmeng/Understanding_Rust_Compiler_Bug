plain
2019-12-08T02:13:14.4499523Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T02:13:15.2563406Z ##[command]git config gc.auto 0
2019-12-08T02:13:15.2567499Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T02:13:15.2569566Z ##[command]git config --get-all http.proxy
2019-12-08T02:13:15.2571924Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-08T02:46:54.0207027Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-08T02:46:58.1350458Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-12-08T02:47:00.2988165Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-08T02:47:14.1850313Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-08T02:47:14.7177917Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:14.7178275Z   --> src/libsyntax_pos/lib.rs:75:10
2019-12-08T02:47:14.7178866Z 75 |          RustcDecodable, RustcEncodable, HashStable_Generic)]
2019-12-08T02:47:14.7179395Z    |          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T02:47:14.7180044Z    | 
2019-12-08T02:47:14.7180324Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:14.7180324Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:14.7180542Z    |
2019-12-08T02:47:14.7180831Z 1  | ($ item : item) => { }
2019-12-08T02:47:14.7181175Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:14.7181396Z    |
2019-12-08T02:47:14.7181864Z    = note: for more information, see ***/issues/47809
2019-12-08T02:47:14.7182202Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:14.7182273Z 
2019-12-08T02:47:14.8137192Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:14.8137554Z    --> src/libsyntax_pos/lib.rs:802:39
2019-12-08T02:47:14.8137811Z     |
2019-12-08T02:47:14.8138143Z 802 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
2019-12-08T02:47:14.8138768Z     | 
2019-12-08T02:47:14.8139521Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:14.8139812Z     |
2019-12-08T02:47:14.8139812Z     |
2019-12-08T02:47:14.8140342Z 1   | ($ item : item) => { }
2019-12-08T02:47:14.8140668Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:14.8140893Z     |
2019-12-08T02:47:14.8141281Z     = note: for more information, see ***/issues/47809
2019-12-08T02:47:14.8141590Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:14.8141649Z 
2019-12-08T02:47:15.0416165Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.0416550Z  --> src/libsyntax_pos/edition.rs:9:26
2019-12-08T02:47:15.0416837Z   |
2019-12-08T02:47:15.0417170Z 9 |          RustcEncodable, RustcDecodable, Eq, HashStable_Generic)]
2019-12-08T02:47:15.0417923Z   | 
2019-12-08T02:47:15.0418190Z  ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.0418420Z   |
2019-12-08T02:47:15.0418420Z   |
2019-12-08T02:47:15.0418682Z 1 | ($ item : item) => { }
2019-12-08T02:47:15.0419014Z   | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.0419252Z   |
2019-12-08T02:47:15.0419652Z   = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.0420342Z   = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.0420403Z 
2019-12-08T02:47:15.0554060Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.0554648Z    |
2019-12-08T02:47:15.0554957Z 63 |          RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T02:47:15.0555319Z    |                          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T02:47:15.0555571Z    | 
2019-12-08T02:47:15.0555571Z    | 
2019-12-08T02:47:15.0555849Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.0556082Z    |
2019-12-08T02:47:15.0556371Z 1  | ($ item : item) => { }
2019-12-08T02:47:15.0557035Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.0557255Z    |
2019-12-08T02:47:15.0557596Z    = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.0557892Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.0557948Z 
2019-12-08T02:47:15.1185194Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.1186363Z     |
2019-12-08T02:47:15.1186689Z 689 | #[derive(Clone, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T02:47:15.1187400Z     |                                        ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T02:47:15.1187818Z     | 
2019-12-08T02:47:15.1187818Z     | 
2019-12-08T02:47:15.1188303Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.1188524Z     |
2019-12-08T02:47:15.1188983Z 1   | ($ item : item) => { }
2019-12-08T02:47:15.1189326Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.1189571Z     |
2019-12-08T02:47:15.1189943Z     = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.1190260Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.1190320Z 
2019-12-08T02:47:15.1252488Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.1253503Z     |
2019-12-08T02:47:15.1253817Z 713 | #[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable,
2019-12-08T02:47:15.1254177Z     |                                                      ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T02:47:15.1254623Z     | 
2019-12-08T02:47:15.1254623Z     | 
2019-12-08T02:47:15.1254887Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.1255121Z     |
2019-12-08T02:47:15.1255379Z 1   | ($ item : item) => { }
2019-12-08T02:47:15.1255885Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.1256360Z     |
2019-12-08T02:47:15.1257081Z     = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.1257437Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.1257489Z 
2019-12-08T02:47:15.1325477Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.1326104Z     |
2019-12-08T02:47:15.1326104Z     |
2019-12-08T02:47:15.1326448Z 749 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T02:47:15.1327083Z     | 
2019-12-08T02:47:15.1327525Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.1327998Z     |
2019-12-08T02:47:15.1327998Z     |
2019-12-08T02:47:15.1328263Z 1   | ($ item : item) => { }
2019-12-08T02:47:15.1328598Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.1328834Z     |
2019-12-08T02:47:15.1329202Z     = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.1329537Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.1329578Z 
2019-12-08T02:47:15.1426870Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T02:47:15.1427775Z     |
2019-12-08T02:47:15.1427775Z     |
2019-12-08T02:47:15.1428392Z 767 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T02:47:15.1429266Z     | 
2019-12-08T02:47:15.1429563Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T02:47:15.1429785Z     |
2019-12-08T02:47:15.1429785Z     |
2019-12-08T02:47:15.1430054Z 1   | ($ item : item) => { }
2019-12-08T02:47:15.1430430Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T02:47:15.1430862Z     |
2019-12-08T02:47:15.1431401Z     = note: for more information, see ***/issues/47809
2019-12-08T02:47:15.1431736Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T02:47:15.2293657Z error: aborting due to 8 previous errors
2019-12-08T02:47:15.2294387Z 
2019-12-08T02:47:15.2295033Z For more information about this error, try `rustc --explain E0658`.
2019-12-08T02:47:15.2466237Z error: could not compile `syntax_pos`.
---
2019-12-08T02:47:15.2575749Z   local time: Sun Dec  8 02:47:15 UTC 2019
2019-12-08T02:47:15.8017937Z   network time: Sun, 08 Dec 2019 02:47:15 GMT
2019-12-08T02:47:15.8028081Z == end clock drift check ==
2019-12-08T02:47:17.2491181Z 
2019-12-08T02:47:17.2606427Z ##[error]Bash exited with code '1'.
2019-12-08T02:47:17.2649012Z ##[section]Starting: Checkout
2019-12-08T02:47:17.2650717Z ==============================================================================
2019-12-08T02:47:17.2650867Z Task         : Get sources
2019-12-08T02:47:17.2650928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
