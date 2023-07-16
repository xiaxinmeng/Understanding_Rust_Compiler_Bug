plain
2019-12-24T01:06:23.0782803Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-24T01:06:23.0797645Z ##[command]git config gc.auto 0
2019-12-24T01:06:23.0801245Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-24T01:06:23.0804165Z ##[command]git config --get-all http.proxy
2019-12-24T01:06:23.0807132Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-24T01:38:59.1360427Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-24T01:39:02.6962640Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-12-24T01:39:05.1967506Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-24T01:39:17.5548285Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-24T01:39:18.0718975Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.0719454Z   --> src/libsyntax_pos/lib.rs:81:5
2019-12-24T01:39:18.0720213Z 81 |     RustcDecodable,
2019-12-24T01:39:18.0721249Z    |     ^^^^^^^^^^^^^^ in this macro invocation
2019-12-24T01:39:18.0721518Z    | 
2019-12-24T01:39:18.0721808Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.0721808Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.0722086Z    |
2019-12-24T01:39:18.0722375Z 1  | ($ item : item) => { }
2019-12-24T01:39:18.0722734Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.0722983Z    |
2019-12-24T01:39:18.0723448Z    = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.0723781Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.0723844Z 
2019-12-24T01:39:18.6234220Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6234800Z    --> src/libsyntax_pos/lib.rs:796:39
2019-12-24T01:39:18.6235235Z     |
2019-12-24T01:39:18.6235678Z 796 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
2019-12-24T01:39:18.6236878Z     | 
2019-12-24T01:39:18.6237282Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6237647Z     |
2019-12-24T01:39:18.6237647Z     |
2019-12-24T01:39:18.6238470Z 1   | ($ item : item) => { }
2019-12-24T01:39:18.6239433Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6240641Z     |
2019-12-24T01:39:18.6241324Z     = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6241866Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6242047Z 
2019-12-24T01:39:18.6244834Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6245250Z   --> src/libsyntax_pos/edition.rs:16:5
2019-12-24T01:39:18.6246007Z 16 |     RustcDecodable,
2019-12-24T01:39:18.6246442Z    |     ^^^^^^^^^^^^^^ in this macro invocation
2019-12-24T01:39:18.6247048Z    | 
2019-12-24T01:39:18.6247611Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6247611Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6248083Z    |
2019-12-24T01:39:18.6248503Z 1  | ($ item : item) => { }
2019-12-24T01:39:18.6248932Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6249288Z    |
2019-12-24T01:39:18.6249730Z    = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6250951Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6251158Z 
2019-12-24T01:39:18.6251742Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6252673Z    |
2019-12-24T01:39:18.6253141Z 71 |     RustcDecodable,
2019-12-24T01:39:18.6253649Z    |     ^^^^^^^^^^^^^^ in this macro invocation
2019-12-24T01:39:18.6254221Z    | 
2019-12-24T01:39:18.6254221Z    | 
2019-12-24T01:39:18.6254803Z   ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6255129Z    |
2019-12-24T01:39:18.6255512Z 1  | ($ item : item) => { }
2019-12-24T01:39:18.6255937Z    | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6256277Z    |
2019-12-24T01:39:18.6256740Z    = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6257156Z    = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6257326Z 
2019-12-24T01:39:18.6257945Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6258707Z     |
2019-12-24T01:39:18.6259116Z 720 | #[derive(Clone, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-24T01:39:18.6259586Z     |                                        ^^^^^^^^^^^^^^ in this macro invocation
2019-12-24T01:39:18.6260121Z     | 
2019-12-24T01:39:18.6260121Z     | 
2019-12-24T01:39:18.6261030Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6261476Z     |
2019-12-24T01:39:18.6261929Z 1   | ($ item : item) => { }
2019-12-24T01:39:18.6262487Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6262897Z     |
2019-12-24T01:39:18.6263458Z     = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6264126Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6264425Z 
2019-12-24T01:39:18.6265165Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6265942Z     |
2019-12-24T01:39:18.6266309Z 750 |     RustcDecodable,
2019-12-24T01:39:18.6266735Z     |     ^^^^^^^^^^^^^^ in this macro invocation
2019-12-24T01:39:18.6267083Z     | 
2019-12-24T01:39:18.6267083Z     | 
2019-12-24T01:39:18.6267458Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6267805Z     |
2019-12-24T01:39:18.6268172Z 1   | ($ item : item) => { }
2019-12-24T01:39:18.6268614Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6268965Z     |
2019-12-24T01:39:18.6269401Z     = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6269839Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6270240Z 
2019-12-24T01:39:18.6271244Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6271762Z     |
2019-12-24T01:39:18.6271762Z     |
2019-12-24T01:39:18.6272339Z 789 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-24T01:39:18.6273376Z     | 
2019-12-24T01:39:18.6273843Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6274610Z     |
2019-12-24T01:39:18.6274610Z     |
2019-12-24T01:39:18.6274912Z 1   | ($ item : item) => { }
2019-12-24T01:39:18.6275204Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6275619Z     |
2019-12-24T01:39:18.6276568Z     = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6276928Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6276966Z 
2019-12-24T01:39:18.6277382Z error[E0658]: use of unstable library feature 'track_caller': uses #[track_caller] which is not yet stable
2019-12-24T01:39:18.6278346Z     |
2019-12-24T01:39:18.6278346Z     |
2019-12-24T01:39:18.6280612Z 807 | #[derive(Clone, Copy, PartialEq, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-12-24T01:39:18.6281526Z     | 
2019-12-24T01:39:18.6281837Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-12-24T01:39:18.6282146Z     |
2019-12-24T01:39:18.6282146Z     |
2019-12-24T01:39:18.6283427Z 1   | ($ item : item) => { }
2019-12-24T01:39:18.6284320Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-12-24T01:39:18.6284669Z     |
2019-12-24T01:39:18.6286741Z     = note: for more information, see ***/issues/47809
2019-12-24T01:39:18.6287677Z     = help: add `#![feature(track_caller)]` to the crate attributes to enable
2019-12-24T01:39:18.6288210Z error: aborting due to 8 previous errors
2019-12-24T01:39:18.6288375Z 
2019-12-24T01:39:18.6288751Z For more information about this error, try `rustc --explain E0658`.
2019-12-24T01:39:18.6288993Z error: could not compile `syntax_pos`.
---
2019-12-24T01:39:18.6291681Z   local time: Tue Dec 24 01:39:18 UTC 2019
2019-12-24T01:39:18.7357939Z   network time: Tue, 24 Dec 2019 01:39:18 GMT
2019-12-24T01:39:18.7358640Z == end clock drift check ==
2019-12-24T01:39:19.8735416Z 
2019-12-24T01:39:19.8835741Z ##[error]Bash exited with code '1'.
2019-12-24T01:39:19.8869241Z ##[section]Starting: Checkout
2019-12-24T01:39:19.8871599Z ==============================================================================
2019-12-24T01:39:19.8871660Z Task         : Get sources
2019-12-24T01:39:19.8871708Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
