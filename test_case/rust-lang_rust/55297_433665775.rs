plain
travis_time:end:0602586a:start=1540684739814040976,finish=1540684740999024716,duration=1184983740
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:14:21]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:16:20]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:16:29]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:16:29]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:16:31] error[E0599]: no method named `gensym` found for type `syntax::ast::Symbol` in the current scope
[00:16:31]    --> librustc_resolve/build_reduced_graph.rs:414:73
[00:16:31]     |
[00:16:31] 414 |                             Some(Ident::new(keywords::Underscore.name().gensym(), use_tree.span)),
[00:16:31]     |                                             |                           |
[00:16:31]     |                                             |                           this is an associated function, not a method
[00:16:31]     |                                             |                           this is an associated function, not a method
[00:16:31]     |                                             help: use associated function syntax instead: `syntax::ast::Symbol::gensym`
[00:16:31]     |
[00:16:31]     = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[00:16:31]     = note: the candidate is defined in an impl for the type `syntax::ast::Symbol`
[00:16:31]     = help: did you mean `gensymed`?
[00:16:32] error: aborting due to previous error
[00:16:32] 
[00:16:32] For more information about this error, try `rustc --explain E0599`.
[00:16:32] error: Could not compile `rustc_resolve`.
