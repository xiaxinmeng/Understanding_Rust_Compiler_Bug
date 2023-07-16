plain
travis_time:end:0d006fe3:start=1543278764668816830,finish=1543278765752897792,duration=1084080962
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:16:33]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:16:35] error[E0308]: match arms have incompatible types
[00:16:35]    --> src/librustc_resolve/resolve_imports.rs:163:22
[00:16:35]     |
[00:16:35] 163 |           let module = match module {
[00:16:35]     |  ______________________^
[00:16:35] 164 | |             ModuleOrUniformRoot::Module(module) => module,
[00:16:35] 165 | |             ModuleOrUniformRoot::UniformRoot(uniform_root_kind) => {
[00:16:35] 166 | |                 assert!(!restricted_shadowing);
[00:16:35] 206 | |             }
[00:16:35] 207 | |         };
[00:16:35]     | |_________^ expected reference, found enum `std::result::Result`
[00:16:35]     |
[00:16:35]     |
[00:16:35]     = note: expected type `&ModuleData<'_>`
[00:16:35]                found type `std::result::Result<&NameBinding<'_>, (syntax::ext::base::Determinacy, Weak)>`
[00:16:35] note: match arm with an incompatible type
[00:16:35]    --> src/librustc_resolve/resolve_imports.rs:165:68
[00:16:35]     |
[00:16:35] 165 |               ModuleOrUniformRoot::UniformRoot(uniform_root_kind) => {
[00:16:35]     |  ____________________________________________________________________^
[00:16:35] 166 | |                 assert!(!restricted_shadowing);
[00:16:35] 167 | |                 match uniform_root_kind {
[00:16:35] 168 | |                     UniformRootKind::ExternPrelude => {
[00:16:35] 205 | |                 }
[00:16:35] 206 | |             }
[00:16:35]     | |_____________^
[00:16:35] 
