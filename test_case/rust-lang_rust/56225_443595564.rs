plain
travis_time:end:250c3ff0:start=1543813598670301959,finish=1543813599830021522,duration=1159719563
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:36:59]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:37:02] error[E0609]: no field `name` on type `&rustc::ty::VariantDef`
[00:37:02]     --> src/librustdoc/clean/mod.rs:2968:29
[00:37:02]      |
[00:37:02] 2968 |             name: Some(self.name.clean(cx)),
[00:37:02] 
[00:37:05] error[E0609]: no field `name` on type `&rustc::ty::VariantDef`
[00:37:05]    --> src/librustdoc/passes/collect_intra_doc_links.rs:605:50
[00:37:05]     |
[00:37:05]     |
[00:37:05] 605 |     Ok((parent_def, Some(format!("{}.v", variant.name))))
[00:37:05] 
[00:37:05] error[E0609]: no field `name` on type `rustc::hir::VariantKind`
[00:37:05]    --> src/librustdoc/visit_ast.rs:155:30
[00:37:05]     |
[00:37:05]     |
[00:37:05] 155 |                 name: v.node.name,
[00:37:05]     |                              ^^^^ unknown field
[00:37:05]     |
[00:37:05]     = note: available fields are: `ident`, `attrs`, `data`, `disr_expr`
[00:37:05] 
[00:37:05] error[E0560]: struct `rustc::hir::ForeignItem` has no field named `name`
[00:37:05]    --> src/librustdoc/visit_ast.rs:373:25
[00:37:05]     |
[00:37:05] 373 |                         name: renamed.unwrap_or(it.name),
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00034e5f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 05:43:54 UTC 2018
