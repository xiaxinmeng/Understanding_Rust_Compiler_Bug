plain
travis_time:end:042f02b0:start=1551733367644004996,finish=1551733502759830753,duration=135115825757
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:23]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:30]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:55]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:55]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:57] error[E0277]: the trait bound `syntax::tokenstream::TokenStream: std::convert::From<(syntax::tokenstream::TokenStream, std::vec::Vec<syntax::parse::lexer::UnmatchedBrace>)>` is not satisfied
[00:12:57]    --> src/librustc_metadata/cstore_impl.rs:460:30
[00:12:57] 460 |                 tokens: body.into(),
[00:12:57] 460 |                 tokens: body.into(),
[00:12:57]     |                              ^^^^ the trait `std::convert::From<(syntax::tokenstream::TokenStream, std::vec::Vec<syntax::parse::lexer::UnmatchedBrace>)>` is not implemented for `syntax::tokenstream::TokenStream`
[00:12:57]     = help: the following implementations were found:
[00:12:57]     = help: the following implementations were found:
[00:12:57]               <syntax::tokenstream::TokenStream as std::convert::From<syntax::parse::token::Token>>
[00:12:57]               <syntax::tokenstream::TokenStream as std::convert::From<syntax::tokenstream::TokenTree>>
[00:12:57]     = note: required because of the requirements on the impl of `std::convert::Into<syntax::tokenstream::TokenStream>` for `(syntax::tokenstream::TokenStream, std::vec::Vec<syntax::parse::lexer::UnmatchedBrace>)`
[00:12:58] error: aborting due to previous error
[00:12:58] 
[00:12:58] For more information about this error, try `rustc --explain E0277`.
[00:12:58] error: Could not compile `rustc_metadata`.
