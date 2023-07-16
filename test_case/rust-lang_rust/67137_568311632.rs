plain
2019-12-22T22:46:41.9237524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T22:46:42.8274171Z ##[command]git config gc.auto 0
2019-12-22T22:46:42.8282259Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T22:46:42.8285797Z ##[command]git config --get-all http.proxy
2019-12-22T22:46:42.8289557Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-22T23:20:10.8219838Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-22T23:20:14.8566698Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-12-22T23:20:16.4510074Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-22T23:20:30.1601222Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-22T23:20:30.6833711Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:30.6834091Z   --> src/libsyntax_pos/lib.rs:75:10
2019-12-22T23:20:30.6834626Z 75 |          RustcDecodable, RustcEncodable, HashStable_Generic)]
2019-12-22T23:20:30.6834946Z    |          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-22T23:20:30.6835156Z    | 
2019-12-22T23:20:30.6835408Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:30.6835408Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:30.6835622Z    |
2019-12-22T23:20:30.6835860Z 1  | ($ item : item) => { }
2019-12-22T23:20:30.6836174Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:30.6837306Z    |
2019-12-22T23:20:30.6837771Z    = note: for more information, see ***/issues/47809
2019-12-22T23:20:30.6838097Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:30.6838161Z 
2019-12-22T23:20:30.7680853Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:30.7681199Z    --> src/libsyntax_pos/lib.rs:802:39
2019-12-22T23:20:30.7681472Z     |
2019-12-22T23:20:30.7681779Z 802 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
2019-12-22T23:20:30.7682452Z     | 
2019-12-22T23:20:30.7682713Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:30.7682934Z     |
2019-12-22T23:20:30.7682934Z     |
2019-12-22T23:20:30.7683183Z 1   | ($ item : item) => { }
2019-12-22T23:20:30.7683501Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:30.7683724Z     |
2019-12-22T23:20:30.7684115Z     = note: for more information, see ***/issues/47809
2019-12-22T23:20:30.7684582Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:30.7684928Z 
2019-12-22T23:20:31.3854940Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3858931Z  --> src/libsyntax_pos/edition.rs:9:26
2019-12-22T23:20:31.3859263Z   |
2019-12-22T23:20:31.3859613Z 9 |          RustcEncodable, RustcDecodable, Eq, HashStable_Generic)]
2019-12-22T23:20:31.3860375Z   | 
2019-12-22T23:20:31.3860640Z  ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3860866Z   |
2019-12-22T23:20:31.3860866Z   |
2019-12-22T23:20:31.3861136Z 1 | ($ item : item) => { }
2019-12-22T23:20:31.3861467Z   | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3861689Z   |
2019-12-22T23:20:31.3862055Z   = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3862556Z   = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3862596Z 
2019-12-22T23:20:31.3862906Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3863631Z    |
2019-12-22T23:20:31.3863938Z 63 |          RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-22T23:20:31.3864308Z    |                          ^^^^^^^^^^^^^^ in this macro invocation
2019-12-22T23:20:31.3864531Z    | 
2019-12-22T23:20:31.3864531Z    | 
2019-12-22T23:20:31.3864802Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3865033Z    |
2019-12-22T23:20:31.3865296Z 1  | ($ item : item) => { }
2019-12-22T23:20:31.3865638Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3865875Z    |
2019-12-22T23:20:31.3866211Z    = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3867161Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3867225Z 
2019-12-22T23:20:31.3867546Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3868047Z     |
2019-12-22T23:20:31.3868414Z 689 | #[derive(Clone, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-22T23:20:31.3868782Z     |                                        ^^^^^^^^^^^^^^ in this macro invocation
2019-12-22T23:20:31.3869028Z     | 
2019-12-22T23:20:31.3869028Z     | 
2019-12-22T23:20:31.3869597Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3869955Z     |
2019-12-22T23:20:31.3870241Z 1   | ($ item : item) => { }
2019-12-22T23:20:31.3870582Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3870822Z     |
2019-12-22T23:20:31.3871179Z     = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3871499Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3871558Z 
2019-12-22T23:20:31.3872197Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3873183Z     |
2019-12-22T23:20:31.3873498Z 713 | #[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable,
2019-12-22T23:20:31.3873869Z     |                                                      ^^^^^^^^^^^^^^ in this macro invocation
2019-12-22T23:20:31.3874114Z     | 
2019-12-22T23:20:31.3874114Z     | 
2019-12-22T23:20:31.3874389Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3874605Z     |
2019-12-22T23:20:31.3874889Z 1   | ($ item : item) => { }
2019-12-22T23:20:31.3875236Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3875468Z     |
2019-12-22T23:20:31.3875839Z     = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3876355Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3876552Z 
2019-12-22T23:20:31.3876915Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3877415Z     |
2019-12-22T23:20:31.3877415Z     |
2019-12-22T23:20:31.3877752Z 749 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-22T23:20:31.3878395Z     | 
2019-12-22T23:20:31.3878669Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3878881Z     |
2019-12-22T23:20:31.3878881Z     |
2019-12-22T23:20:31.3879167Z 1   | ($ item : item) => { }
2019-12-22T23:20:31.3879508Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3879728Z     |
2019-12-22T23:20:31.3880093Z     = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3880557Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3880685Z 
2019-12-22T23:20:31.3881033Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-22T23:20:31.3881885Z     |
2019-12-22T23:20:31.3881885Z     |
2019-12-22T23:20:31.3882235Z 767 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-22T23:20:31.3882856Z     | 
2019-12-22T23:20:31.3883143Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-22T23:20:31.3883370Z     |
2019-12-22T23:20:31.3883370Z     |
2019-12-22T23:20:31.3883651Z 1   | ($ item : item) => { }
2019-12-22T23:20:31.3883994Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-22T23:20:31.3884213Z     |
2019-12-22T23:20:31.3884584Z     = note: for more information, see ***/issues/47809
2019-12-22T23:20:31.3884905Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-22T23:20:31.3885207Z error: aborting due to 8 previous errors
2019-12-22T23:20:31.3885241Z 
2019-12-22T23:20:31.3885499Z For more information about this error, try `rustc --explain E0658`.
2019-12-22T23:20:31.3885771Z error: could not compile `syntax_pos`.
---
2019-12-22T23:20:31.3887259Z   local time: Sun Dec 22 23:20:31 UTC 2019
2019-12-22T23:20:31.4964665Z   network time: Sun, 22 Dec 2019 23:20:31 GMT
2019-12-22T23:20:31.4964779Z == end clock drift check ==
2019-12-22T23:20:32.8653616Z 
2019-12-22T23:20:32.8745517Z ##[error]Bash exited with code '1'.
2019-12-22T23:20:32.8784073Z ##[section]Starting: Checkout
2019-12-22T23:20:32.8786117Z ==============================================================================
2019-12-22T23:20:32.8786173Z Task         : Get sources
2019-12-22T23:20:32.8786218Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
