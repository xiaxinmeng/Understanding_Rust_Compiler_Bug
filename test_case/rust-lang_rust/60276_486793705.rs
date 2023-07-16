plain
travis_time:end:042b15a0:start=1556217421819619941,finish=1556217538059248633,duration=116239628692
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:53]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:00]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:04]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:07]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:09] error: expected one of `)`, `,`, or `@`, found `:`
[00:08:09]    --> src/librustc/mir/visit.rs:606:39
[00:08:09]     |
[00:08:09] 69  | / macro_rules! make_mir_visitor {
[00:08:09] 70  | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
[00:08:09] 71  | |         pub trait $visitor_trait_name<'tcx> {
[00:08:09] 72  | |             // Override these, and call `self.super_xxx` to revert back to the
[00:08:09] 606 | |                                 def_id: _,
[00:08:09] 606 | |                                 def_id: _,
[00:08:09]     | |                                       ^ expected one of `)`, `,`, or `@` here
[00:08:09] 842 | |     }
[00:08:09] 843 | | }
[00:08:09]     | |_- in this expansion of `make_mir_visitor!`
[00:08:09] 844 | 
[00:08:09] 844 | 
[00:08:09] 845 |   make_mir_visitor!(Visitor,);
[00:08:09] 
[00:08:09] 
[00:08:09] error: expected one of `)`, `,`, or `@`, found `:`
[00:08:09]    --> src/librustc/mir/visit.rs:606:39
[00:08:09]     |
[00:08:09] 69  | / macro_rules! make_mir_visitor {
[00:08:09] 70  | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
[00:08:09] 71  | |         pub trait $visitor_trait_name<'tcx> {
[00:08:09] 72  | |             // Override these, and call `self.super_xxx` to revert back to the
[00:08:09] 606 | |                                 def_id: _,
[00:08:09] 606 | |                                 def_id: _,
[00:08:09]     | |                                       ^ expected one of `)`, `,`, or `@` here
[00:08:09] 842 | |     }
[00:08:09] 843 | | }
[00:08:09]     | |_- in this expansion of `make_mir_visitor!`
[00:08:09] ...
[00:08:09] ...
[00:08:09] 846 |   make_mir_visitor!(MutVisitor,mut);
[00:08:09] 
[00:08:09] 
[00:08:14] error: unused import: `crate::hir::def_id::DefId`
[00:08:14]  --> src/librustc/mir/visit.rs:1:5
[00:08:14] 1 | use crate::hir::def_id::DefId;
[00:08:14]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:08:14]   |
[00:08:14]   = note: `-D unused-imports` implied by `-D warnings`
