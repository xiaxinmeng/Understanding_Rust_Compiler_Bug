plain
travis_time:end:2a7cc96e:start=1558293500505906466,finish=1558293501296013601,duration=790107135
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:29]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:07:41]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:52]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:56]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:06] error[E0277]: `syntax_pos::span_encoding::Span` cannot be shared between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `syntax_pos::span_encoding::Span` cannot be shared between threads safely
[00:08:06]    = help: the trait `std::marker::Sync` is not implemented for `syntax_pos::span_encoding::Span`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] 
[00:08:06] error[E0277]: `syntax_pos::span_encoding::Span` cannot be sent between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `syntax_pos::span_encoding::Span` cannot be sent between threads safely
[00:08:06]    = help: the trait `std::marker::Send` is not implemented for `syntax_pos::span_encoding::Span`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] 
[00:08:06] error[E0277]: `std::rc::Rc<parse::token::Nonterminal>` cannot be shared between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `std::rc::Rc<parse::token::Nonterminal>` cannot be shared between threads safely
[00:08:06]    |
[00:08:06]    = help: within `parse::token::Token`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<parse::token::Nonterminal>`
[00:08:06]    = note: required because it appears within the type `parse::token::Token`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] 
[00:08:06] error[E0277]: `std::rc::Rc<parse::token::Nonterminal>` cannot be sent between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `std::rc::Rc<parse::token::Nonterminal>` cannot be sent between threads safely
[00:08:06]    |
[00:08:06]    = help: within `parse::token::Token`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<parse::token::Nonterminal>`
[00:08:06]    = note: required because it appears within the type `parse::token::Token`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] 
[00:08:06] error[E0277]: `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>` cannot be shared between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>` cannot be shared between threads safely
[00:08:06]    |
[00:08:06]    = help: within `tokenstream::TokenStream`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>`
[00:08:06]    = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>>`
[00:08:06]    = note: required because it appears within the type `tokenstream::TokenStream`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] 
[00:08:06] error[E0277]: `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>` cannot be sent between threads safely
[00:08:06]   --> src/libsyntax/tokenstream.rs:54:1
[00:08:06] 54 | / fn dummy()
[00:08:06] 55 | | where
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 56 | |     Span: Send + Sync,
[00:08:06] 57 | |     token::Token: Send + Sync,
[00:08:06] ...  |
[00:08:06] 60 | |     TokenStream: Send + Sync,
[00:08:06] 61 | | {}
[00:08:06]    | |__^ `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>` cannot be sent between threads safely
[00:08:06]    |
[00:08:06]    = help: within `tokenstream::TokenStream`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>`
[00:08:06]    = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<(tokenstream::TokenTree, tokenstream::IsJoint)>>>`
[00:08:06]    = note: required because it appears within the type `tokenstream::TokenStream`
[00:08:06]    = help: see issue #48214
[00:08:06]    = help: add #![feature(trivial_bounds)] to the crate attributes to enable
[00:08:06] error: aborting due to 6 previous errors
[00:08:06] 
[00:08:06] For more information about this error, try `rustc --explain E0277`.
[00:08:06] error: Could not compile `syntax`.
---
205996 ./obj/build/cache/2019-04-11
157460 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
142472 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9
142468 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9/s-fcda8kt4rb-umvqm1-34hadj2q5a3fu
108532 ./src/llvm-project/lldb
107952 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
101812 ./.git
97584 ./src/llvm-project/clang/test
