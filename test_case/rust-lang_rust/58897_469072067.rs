plain
travis_time:end:0c298965:start=1551652260712900834,finish=1551652261810984898,duration=1098084064
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:01] 
[00:02:04] error[E0308]: mismatched types
[00:02:04]    --> src/bootstrap/tool.rs:359:93
[00:02:04]     |
[00:02:04] 359 |         add_lib_path(vec![PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host))], cmd);
[00:02:04]     |                                                                                             |
[00:02:04]     |                                                                                             expected mutable reference, found struct `std::process::Command`
[00:02:04]     |                                                                                             help: consider mutably borrowing here: `&mut cmd`
[00:02:04]     |
---
[00:02:07] 
[00:02:10] error[E0308]: mismatched types
[00:02:10]    --> src/bootstrap/tool.rs:359:93
[00:02:10]     |
[00:02:10] 359 |         add_lib_path(vec![PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host))], cmd);
[00:02:10]     |                                                                                             |
[00:02:10]     |                                                                                             expected mutable reference, found struct `std::process::Command`
[00:02:10]     |                                                                                             help: consider mutably borrowing here: `&mut cmd`
[00:02:10]     |
---
[00:02:13] 
[00:02:16] error[E0308]: mismatched types
[00:02:16]    --> src/bootstrap/tool.rs:359:93
[00:02:16]     |
[00:02:16] 359 |         add_lib_path(vec![PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host))], cmd);
[00:02:16]     |                                                                                             |
[00:02:16]     |                                                                                             expected mutable reference, found struct `std::process::Command`
[00:02:16]     |                                                                                             help: consider mutably borrowing here: `&mut cmd`
[00:02:16]     |
---
[00:02:20] 
[00:02:23] error[E0308]: mismatched types
[00:02:23]    --> src/bootstrap/tool.rs:359:93
[00:02:23]     |
[00:02:23] 359 |         add_lib_path(vec![PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host))], cmd);
[00:02:23]     |                                                                                             |
[00:02:23]     |                                                                                             expected mutable reference, found struct `std::process::Command`
[00:02:23]     |                                                                                             help: consider mutably borrowing here: `&mut cmd`
[00:02:23]     |
---
[00:02:29] 
[00:02:32] error[E0308]: mismatched types
[00:02:32]    --> src/bootstrap/tool.rs:359:93
[00:02:32]     |
[00:02:32] 359 |         add_lib_path(vec![PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host))], cmd);
[00:02:32]     |                                                                                             |
[00:02:32]     |                                                                                             expected mutable reference, found struct `std::process::Command`
[00:02:32]     |                                                                                             help: consider mutably borrowing here: `&mut cmd`
[00:02:32]     |
