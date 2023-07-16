plain
2019-12-08T13:51:35.4830670Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T13:51:35.4849442Z ##[command]git config gc.auto 0
2019-12-08T13:51:35.4853028Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T13:51:35.4859177Z ##[command]git config --get-all http.proxy
2019-12-08T13:51:35.4864808Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-08T14:19:42.5088934Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-08T14:19:45.8398616Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-12-08T14:19:48.2744682Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-08T14:19:59.4200671Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-08T14:19:59.8412100Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:19:59.8412458Z   --> src/libsyntax_pos/lib.rs:75:10
2019-12-08T14:19:59.8412958Z 75 |          RustcDecodable, RustcEncodable, HashStable_Generic)]
2019-12-08T14:19:59.8413267Z    |          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T14:19:59.8413476Z    | 
2019-12-08T14:19:59.8413730Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:19:59.8413730Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:19:59.8413944Z    |
2019-12-08T14:19:59.8414180Z 1  | ($ item : item) => { }
2019-12-08T14:19:59.8414489Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:19:59.8414715Z    |
2019-12-08T14:19:59.8415106Z    = note: for more information, see ***/issues/47809
2019-12-08T14:19:59.8415414Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:19:59.8415452Z 
2019-12-08T14:19:59.9118629Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:19:59.9118939Z    --> src/libsyntax_pos/lib.rs:802:39
2019-12-08T14:19:59.9119170Z     |
2019-12-08T14:19:59.9119464Z 802 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
2019-12-08T14:19:59.9120051Z     | 
2019-12-08T14:19:59.9120298Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:19:59.9120496Z     |
2019-12-08T14:19:59.9120496Z     |
2019-12-08T14:19:59.9120754Z 1   | ($ item : item) => { }
2019-12-08T14:19:59.9121058Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:19:59.9121273Z     |
2019-12-08T14:19:59.9121612Z     = note: for more information, see ***/issues/47809
2019-12-08T14:19:59.9122079Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:19:59.9122215Z 
2019-12-08T14:20:00.0806585Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.0806890Z  --> src/libsyntax_pos/edition.rs:9:26
2019-12-08T14:20:00.0807124Z   |
2019-12-08T14:20:00.0807405Z 9 |          RustcEncodable, RustcDecodable, Eq, HashStable_Generic)]
2019-12-08T14:20:00.0807936Z   | 
2019-12-08T14:20:00.0808182Z  ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.0808423Z   |
2019-12-08T14:20:00.0808423Z   |
2019-12-08T14:20:00.0808665Z 1 | ($ item : item) => { }
2019-12-08T14:20:00.0808967Z   | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.0809179Z   |
2019-12-08T14:20:00.0809517Z   = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.0809813Z   = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.0809867Z 
2019-12-08T14:20:00.0918349Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.0918954Z    |
2019-12-08T14:20:00.0919254Z 63 |          RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T14:20:00.0919583Z    |                          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T14:20:00.0919820Z    | 
2019-12-08T14:20:00.0919820Z    | 
2019-12-08T14:20:00.0920084Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.0920293Z    |
2019-12-08T14:20:00.0920576Z 1  | ($ item : item) => { }
2019-12-08T14:20:00.0920898Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.0921164Z    |
2019-12-08T14:20:00.0921573Z    = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.0921913Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.0926122Z 
2019-12-08T14:20:00.1446918Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.1447476Z     |
2019-12-08T14:20:00.1447769Z 689 | #[derive(Clone, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T14:20:00.1448111Z     |                                        ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T14:20:00.1448641Z     | 
2019-12-08T14:20:00.1448641Z     | 
2019-12-08T14:20:00.1448932Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.1449149Z     |
2019-12-08T14:20:00.1449392Z 1   | ($ item : item) => { }
2019-12-08T14:20:00.1449698Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.1449913Z     |
2019-12-08T14:20:00.1450265Z     = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.1450558Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.1454204Z 
2019-12-08T14:20:00.1502663Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.1503198Z     |
2019-12-08T14:20:00.1503479Z 713 | #[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable,
2019-12-08T14:20:00.1503828Z     |                                                      ^^^^^^^^^^^^^^ in this macro invocation
2019-12-08T14:20:00.1504042Z     | 
2019-12-08T14:20:00.1504042Z     | 
2019-12-08T14:20:00.1504292Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.1504517Z     |
2019-12-08T14:20:00.1504761Z 1   | ($ item : item) => { }
2019-12-08T14:20:00.1505079Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.1505307Z     |
2019-12-08T14:20:00.1505635Z     = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.1505950Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.1509528Z 
2019-12-08T14:20:00.1567243Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.1567770Z     |
2019-12-08T14:20:00.1567770Z     |
2019-12-08T14:20:00.1568104Z 749 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T14:20:00.1568661Z     | 
2019-12-08T14:20:00.1568924Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.1569124Z     |
2019-12-08T14:20:00.1569124Z     |
2019-12-08T14:20:00.1569361Z 1   | ($ item : item) => { }
2019-12-08T14:20:00.1569679Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.1569880Z     |
2019-12-08T14:20:00.1570362Z     = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.1570772Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.1574044Z 
2019-12-08T14:20:00.1641719Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-08T14:20:00.1642661Z     |
2019-12-08T14:20:00.1642661Z     |
2019-12-08T14:20:00.1643153Z 767 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-08T14:20:00.1644052Z     | 
2019-12-08T14:20:00.1644507Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-08T14:20:00.1644883Z     |
2019-12-08T14:20:00.1644883Z     |
2019-12-08T14:20:00.1645312Z 1   | ($ item : item) => { }
2019-12-08T14:20:00.1645772Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-08T14:20:00.1646161Z     |
2019-12-08T14:20:00.1646653Z     = note: for more information, see ***/issues/47809
2019-12-08T14:20:00.1647140Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-08T14:20:00.2346883Z error: aborting due to 8 previous errors
2019-12-08T14:20:00.2349659Z 
2019-12-08T14:20:00.2354507Z For more information about this error, try `rustc --explain E0658`.
2019-12-08T14:20:00.2491340Z error: could not compile `syntax_pos`.
---
2019-12-08T14:20:00.2624725Z   local time: Sun Dec  8 14:20:00 UTC 2019
2019-12-08T14:20:00.5333123Z   network time: Sun, 08 Dec 2019 14:20:00 GMT
2019-12-08T14:20:00.5335441Z == end clock drift check ==
2019-12-08T14:20:01.9617573Z 
2019-12-08T14:20:01.9685501Z ##[error]Bash exited with code '1'.
2019-12-08T14:20:01.9720241Z ##[section]Starting: Checkout
2019-12-08T14:20:01.9722100Z ==============================================================================
2019-12-08T14:20:01.9722151Z Task         : Get sources
2019-12-08T14:20:01.9722215Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
