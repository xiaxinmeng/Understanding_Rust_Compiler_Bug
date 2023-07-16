plain
travis_time:end:105e076f:start=1549965647168432648,finish=1549965648142243702,duration=973811054
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:28]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:30] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:30]     --> src/bootstrap/check.rs:55:9
[00:04:30]      |
[00:04:30] 55   | /         run_cargo(builder,
[00:04:30] 56   | |                   &mut cargo,
[00:04:30] 57   | |                   args(builder.kind),
[00:04:30] 58   | |                   &libstd_stamp(builder, compiler, target),
[00:04:30] 59   | |                   true);
[00:04:30]      | 
[00:04:30]     ::: src/bootstrap/compile.rs:1006:1
[00:04:30]      |
[00:04:30]      |
[00:04:30] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:30] 1007 | |                  cargo: &mut Command,
[00:04:30] 1008 | |                  stamp: &Path,
[00:04:30] 1009 | |                  is_check: bool)
[00:04:30] 1154 | |     deps
[00:04:30] 1155 | | }
[00:04:30]      | |_- defined here
[00:04:30] 
[00:04:30] 
[00:04:30] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:30]     --> src/bootstrap/check.rs:103:9
[00:04:30]      |
[00:04:30] 103  | /         run_cargo(builder,
[00:04:30] 104  | |                   &mut cargo,
[00:04:30] 105  | |                   args(builder.kind),
[00:04:30] 106  | |                   &librustc_stamp(builder, compiler, target),
[00:04:30] 107  | |                   true);
[00:04:30]      | 
[00:04:30]     ::: src/bootstrap/compile.rs:1006:1
[00:04:30]      |
[00:04:30]      |
[00:04:30] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:30] 1007 | |                  cargo: &mut Command,
[00:04:30] 1008 | |                  stamp: &Path,
[00:04:30] 1009 | |                  is_check: bool)
[00:04:30] 1154 | |     deps
[00:04:30] 1155 | | }
[00:04:30]      | |_- defined here
[00:04:30] 
[00:04:30] 
[00:04:30] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:30]     --> src/bootstrap/check.rs:155:9
[00:04:30]      |
[00:04:30] 155  | /         run_cargo(builder,
[00:04:30] 156  | |                   &mut cargo,
[00:04:30] 157  | |                   args(builder.kind),
[00:04:30] 158  | |                   &codegen_backend_stamp(builder, compiler, target, backend),
[00:04:30] 159  | |                   true);
[00:04:30]      | 
[00:04:30]     ::: src/bootstrap/compile.rs:1006:1
[00:04:30]      |
[00:04:30]      |
[00:04:30] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:30] 1007 | |                  cargo: &mut Command,
[00:04:30] 1008 | |                  stamp: &Path,
[00:04:30] 1009 | |                  is_check: bool)
[00:04:30] 1154 | |     deps
[00:04:30] 1155 | | }
[00:04:30]      | |_- defined here
[00:04:30] 
[00:04:30] 
[00:04:30] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:30]     --> src/bootstrap/check.rs:193:9
[00:04:30]      |
[00:04:30] 193  | /         run_cargo(builder,
[00:04:30] 194  | |                   &mut cargo,
[00:04:30] 195  | |                   args(builder.kind),
[00:04:30] 196  | |                   &libtest_stamp(builder, compiler, target),
[00:04:30] 197  | |                   true);
[00:04:30]      | 
[00:04:30]     ::: src/bootstrap/compile.rs:1006:1
[00:04:30]      |
[00:04:30]      |
[00:04:30] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:30] 1007 | |                  cargo: &mut Command,
[00:04:30] 1008 | |                  stamp: &Path,
[00:04:30] 1009 | |                  is_check: bool)
[00:04:30] 1154 | |     deps
[00:04:30] 1155 | | }
[00:04:30]      | |_- defined here
[00:04:30] 
[00:04:30] 
[00:04:30] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:30]     --> src/bootstrap/check.rs:241:9
[00:04:30]      |
[00:04:30] 241  | /         run_cargo(builder,
[00:04:30] 242  | |                   &mut cargo,
[00:04:30] 243  | |                   args(builder.kind),
[00:04:30] 244  | |                   &rustdoc_stamp(builder, compiler, target),
[00:04:30] 245  | |                   true);
[00:04:30]      | 
[00:04:30]     ::: src/bootstrap/compile.rs:1006:1
[00:04:30]      |
[00:04:30]      |
[00:04:30] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:30] 1007 | |                  cargo: &mut Command,
[00:04:30] 1008 | |                  stamp: &Path,
[00:04:30] 1009 | |                  is_check: bool)
[00:04:30] 1154 | |     deps
[00:04:30] 1155 | | }
[00:04:30]      | |_- defined here
[00:04:30] 
---
[00:04:33]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:35]     --> src/bootstrap/check.rs:55:9
[00:04:35]      |
[00:04:35] 55   | /         run_cargo(builder,
[00:04:35] 56   | |                   &mut cargo,
[00:04:35] 57   | |                   args(builder.kind),
[00:04:35] 58   | |                   &libstd_stamp(builder, compiler, target),
[00:04:35] 59   | |                   true);
[00:04:35]      | 
[00:04:35]     ::: src/bootstrap/compile.rs:1006:1
[00:04:35]      |
[00:04:35]      |
[00:04:35] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:35] 1007 | |                  cargo: &mut Command,
[00:04:35] 1008 | |                  stamp: &Path,
[00:04:35] 1009 | |                  is_check: bool)
[00:04:35] 1154 | |     deps
[00:04:35] 1155 | | }
[00:04:35]      | |_- defined here
[00:04:35] 
[00:04:35] 
[00:04:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:35]     --> src/bootstrap/check.rs:103:9
[00:04:35]      |
[00:04:35] 103  | /         run_cargo(builder,
[00:04:35] 104  | |                   &mut cargo,
[00:04:35] 105  | |                   args(builder.kind),
[00:04:35] 106  | |                   &librustc_stamp(builder, compiler, target),
[00:04:35] 107  | |                   true);
[00:04:35]      | 
[00:04:35]     ::: src/bootstrap/compile.rs:1006:1
[00:04:35]      |
[00:04:35]      |
[00:04:35] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:35] 1007 | |                  cargo: &mut Command,
[00:04:35] 1008 | |                  stamp: &Path,
[00:04:35] 1009 | |                  is_check: bool)
[00:04:35] 1154 | |     deps
[00:04:35] 1155 | | }
[00:04:35]      | |_- defined here
[00:04:35] 
[00:04:35] 
[00:04:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:35]     --> src/bootstrap/check.rs:155:9
[00:04:35]      |
[00:04:35] 155  | /         run_cargo(builder,
[00:04:35] 156  | |                   &mut cargo,
[00:04:35] 157  | |                   args(builder.kind),
[00:04:35] 158  | |                   &codegen_backend_stamp(builder, compiler, target, backend),
[00:04:35] 159  | |                   true);
[00:04:35]      | 
[00:04:35]     ::: src/bootstrap/compile.rs:1006:1
[00:04:35]      |
[00:04:35]      |
[00:04:35] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:35] 1007 | |                  cargo: &mut Command,
[00:04:35] 1008 | |                  stamp: &Path,
[00:04:35] 1009 | |                  is_check: bool)
[00:04:35] 1154 | |     deps
[00:04:35] 1155 | | }
[00:04:35]      | |_- defined here
[00:04:35] 
[00:04:35] 
[00:04:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:35]     --> src/bootstrap/check.rs:193:9
[00:04:35]      |
[00:04:35] 193  | /         run_cargo(builder,
[00:04:35] 194  | |                   &mut cargo,
[00:04:35] 195  | |                   args(builder.kind),
[00:04:35] 196  | |                   &libtest_stamp(builder, compiler, target),
[00:04:35] 197  | |                   true);
[00:04:35]      | 
[00:04:35]     ::: src/bootstrap/compile.rs:1006:1
[00:04:35]      |
[00:04:35]      |
[00:04:35] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:35] 1007 | |                  cargo: &mut Command,
[00:04:35] 1008 | |                  stamp: &Path,
[00:04:35] 1009 | |                  is_check: bool)
[00:04:35] 1154 | |     deps
[00:04:35] 1155 | | }
[00:04:35]      | |_- defined here
[00:04:35] 
[00:04:35] 
[00:04:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:35]     --> src/bootstrap/check.rs:241:9
[00:04:35]      |
[00:04:35] 241  | /         run_cargo(builder,
[00:04:35] 242  | |                   &mut cargo,
[00:04:35] 243  | |                   args(builder.kind),
[00:04:35] 244  | |                   &rustdoc_stamp(builder, compiler, target),
[00:04:35] 245  | |                   true);
[00:04:35]      | 
[00:04:35]     ::: src/bootstrap/compile.rs:1006:1
[00:04:35]      |
[00:04:35]      |
[00:04:35] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:35] 1007 | |                  cargo: &mut Command,
[00:04:35] 1008 | |                  stamp: &Path,
[00:04:35] 1009 | |                  is_check: bool)
[00:04:35] 1154 | |     deps
[00:04:35] 1155 | | }
[00:04:35]      | |_- defined here
[00:04:35] 
---
[00:04:40]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:42] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:42]     --> src/bootstrap/check.rs:55:9
[00:04:42]      |
[00:04:42] 55   | /         run_cargo(builder,
[00:04:42] 56   | |                   &mut cargo,
[00:04:42] 57   | |                   args(builder.kind),
[00:04:42] 58   | |                   &libstd_stamp(builder, compiler, target),
[00:04:42] 59   | |                   true);
[00:04:42]      | 
[00:04:42]     ::: src/bootstrap/compile.rs:1006:1
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:42] 1007 | |                  cargo: &mut Command,
[00:04:42] 1008 | |                  stamp: &Path,
[00:04:42] 1009 | |                  is_check: bool)
[00:04:42] 1154 | |     deps
[00:04:42] 1155 | | }
[00:04:42]      | |_- defined here
[00:04:42] 
[00:04:42] 
[00:04:42] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:42]     --> src/bootstrap/check.rs:103:9
[00:04:42]      |
[00:04:42] 103  | /         run_cargo(builder,
[00:04:42] 104  | |                   &mut cargo,
[00:04:42] 105  | |                   args(builder.kind),
[00:04:42] 106  | |                   &librustc_stamp(builder, compiler, target),
[00:04:42] 107  | |                   true);
[00:04:42]      | 
[00:04:42]     ::: src/bootstrap/compile.rs:1006:1
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:42] 1007 | |                  cargo: &mut Command,
[00:04:42] 1008 | |                  stamp: &Path,
[00:04:42] 1009 | |                  is_check: bool)
[00:04:42] 1154 | |     deps
[00:04:42] 1155 | | }
[00:04:42]      | |_- defined here
[00:04:42] 
[00:04:42] 
[00:04:42] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:42]     --> src/bootstrap/check.rs:155:9
[00:04:42]      |
[00:04:42] 155  | /         run_cargo(builder,
[00:04:42] 156  | |                   &mut cargo,
[00:04:42] 157  | |                   args(builder.kind),
[00:04:42] 158  | |                   &codegen_backend_stamp(builder, compiler, target, backend),
[00:04:42] 159  | |                   true);
[00:04:42]      | 
[00:04:42]     ::: src/bootstrap/compile.rs:1006:1
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:42] 1007 | |                  cargo: &mut Command,
[00:04:42] 1008 | |                  stamp: &Path,
[00:04:42] 1009 | |                  is_check: bool)
[00:04:42] 1154 | |     deps
[00:04:42] 1155 | | }
[00:04:42]      | |_- defined here
[00:04:42] 
[00:04:42] 
[00:04:42] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:42]     --> src/bootstrap/check.rs:193:9
[00:04:42]      |
[00:04:42] 193  | /         run_cargo(builder,
[00:04:42] 194  | |                   &mut cargo,
[00:04:42] 195  | |                   args(builder.kind),
[00:04:42] 196  | |                   &libtest_stamp(builder, compiler, target),
[00:04:42] 197  | |                   true);
[00:04:42]      | 
[00:04:42]     ::: src/bootstrap/compile.rs:1006:1
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:42] 1007 | |                  cargo: &mut Command,
[00:04:42] 1008 | |                  stamp: &Path,
[00:04:42] 1009 | |                  is_check: bool)
[00:04:42] 1154 | |     deps
[00:04:42] 1155 | | }
[00:04:42]      | |_- defined here
[00:04:42] 
[00:04:42] 
[00:04:42] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:42]     --> src/bootstrap/check.rs:241:9
[00:04:42]      |
[00:04:42] 241  | /         run_cargo(builder,
[00:04:42] 242  | |                   &mut cargo,
[00:04:42] 243  | |                   args(builder.kind),
[00:04:42] 244  | |                   &rustdoc_stamp(builder, compiler, target),
[00:04:42] 245  | |                   true);
[00:04:42]      | 
[00:04:42]     ::: src/bootstrap/compile.rs:1006:1
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:42] 1007 | |                  cargo: &mut Command,
[00:04:42] 1008 | |                  stamp: &Path,
[00:04:42] 1009 | |                  is_check: bool)
[00:04:42] 1154 | |     deps
[00:04:42] 1155 | | }
[00:04:42]      | |_- defined here
[00:04:42] 
---
[00:04:47]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:49] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:49]     --> src/bootstrap/check.rs:55:9
[00:04:49]      |
[00:04:49] 55   | /         run_cargo(builder,
[00:04:49] 56   | |                   &mut cargo,
[00:04:49] 57   | |                   args(builder.kind),
[00:04:49] 58   | |                   &libstd_stamp(builder, compiler, target),
[00:04:49] 59   | |                   true);
[00:04:49]      | 
[00:04:49]     ::: src/bootstrap/compile.rs:1006:1
[00:04:49]      |
[00:04:49]      |
[00:04:49] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:49] 1007 | |                  cargo: &mut Command,
[00:04:49] 1008 | |                  stamp: &Path,
[00:04:49] 1009 | |                  is_check: bool)
[00:04:49] 1154 | |     deps
[00:04:49] 1155 | | }
[00:04:49]      | |_- defined here
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:49]     --> src/bootstrap/check.rs:103:9
[00:04:49]      |
[00:04:49] 103  | /         run_cargo(builder,
[00:04:49] 104  | |                   &mut cargo,
[00:04:49] 105  | |                   args(builder.kind),
[00:04:49] 106  | |                   &librustc_stamp(builder, compiler, target),
[00:04:49] 107  | |                   true);
[00:04:49]      | 
[00:04:49]     ::: src/bootstrap/compile.rs:1006:1
[00:04:49]      |
[00:04:49]      |
[00:04:49] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:49] 1007 | |                  cargo: &mut Command,
[00:04:49] 1008 | |                  stamp: &Path,
[00:04:49] 1009 | |                  is_check: bool)
[00:04:49] 1154 | |     deps
[00:04:49] 1155 | | }
[00:04:49]      | |_- defined here
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:49]     --> src/bootstrap/check.rs:155:9
[00:04:49]      |
[00:04:49] 155  | /         run_cargo(builder,
[00:04:49] 156  | |                   &mut cargo,
[00:04:49] 157  | |                   args(builder.kind),
[00:04:49] 158  | |                   &codegen_backend_stamp(builder, compiler, target, backend),
[00:04:49] 159  | |                   true);
[00:04:49]      | 
[00:04:49]     ::: src/bootstrap/compile.rs:1006:1
[00:04:49]      |
[00:04:49]      |
[00:04:49] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:49] 1007 | |                  cargo: &mut Command,
[00:04:49] 1008 | |                  stamp: &Path,
[00:04:49] 1009 | |                  is_check: bool)
[00:04:49] 1154 | |     deps
[00:04:49] 1155 | | }
[00:04:49]      | |_- defined here
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:49]     --> src/bootstrap/check.rs:193:9
[00:04:49]      |
[00:04:49] 193  | /         run_cargo(builder,
[00:04:49] 194  | |                   &mut cargo,
[00:04:49] 195  | |                   args(builder.kind),
[00:04:49] 196  | |                   &libtest_stamp(builder, compiler, target),
[00:04:49] 197  | |                   true);
[00:04:49]      | 
[00:04:49]     ::: src/bootstrap/compile.rs:1006:1
[00:04:49]      |
[00:04:49]      |
[00:04:49] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:49] 1007 | |                  cargo: &mut Command,
[00:04:49] 1008 | |                  stamp: &Path,
[00:04:49] 1009 | |                  is_check: bool)
[00:04:49] 1154 | |     deps
[00:04:49] 1155 | | }
[00:04:49]      | |_- defined here
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:49]     --> src/bootstrap/check.rs:241:9
[00:04:49]      |
[00:04:49] 241  | /         run_cargo(builder,
[00:04:49] 242  | |                   &mut cargo,
[00:04:49] 243  | |                   args(builder.kind),
[00:04:49] 244  | |                   &rustdoc_stamp(builder, compiler, target),
[00:04:49] 245  | |                   true);
[00:04:49]      | 
[00:04:49]     ::: src/bootstrap/compile.rs:1006:1
[00:04:49]      |
[00:04:49]      |
[00:04:49] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:49] 1007 | |                  cargo: &mut Command,
[00:04:49] 1008 | |                  stamp: &Path,
[00:04:49] 1009 | |                  is_check: bool)
[00:04:49] 1154 | |     deps
[00:04:49] 1155 | | }
[00:04:49]      | |_- defined here
[00:04:49] 
---
[00:04:56]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:04:58] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:58]     --> src/bootstrap/check.rs:55:9
[00:04:58]      |
[00:04:58] 55   | /         run_cargo(builder,
[00:04:58] 56   | |                   &mut cargo,
[00:04:58] 57   | |                   args(builder.kind),
[00:04:58] 58   | |                   &libstd_stamp(builder, compiler, target),
[00:04:58] 59   | |                   true);
[00:04:58]      | 
[00:04:58]     ::: src/bootstrap/compile.rs:1006:1
[00:04:58]      |
[00:04:58]      |
[00:04:58] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:58] 1007 | |                  cargo: &mut Command,
[00:04:58] 1008 | |                  stamp: &Path,
[00:04:58] 1009 | |                  is_check: bool)
[00:04:58] 1154 | |     deps
[00:04:58] 1155 | | }
[00:04:58]      | |_- defined here
[00:04:58] 
[00:04:58] 
[00:04:58] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:58]     --> src/bootstrap/check.rs:103:9
[00:04:58]      |
[00:04:58] 103  | /         run_cargo(builder,
[00:04:58] 104  | |                   &mut cargo,
[00:04:58] 105  | |                   args(builder.kind),
[00:04:58] 106  | |                   &librustc_stamp(builder, compiler, target),
[00:04:58] 107  | |                   true);
[00:04:58]      | 
[00:04:58]     ::: src/bootstrap/compile.rs:1006:1
[00:04:58]      |
[00:04:58]      |
[00:04:58] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:58] 1007 | |                  cargo: &mut Command,
[00:04:58] 1008 | |                  stamp: &Path,
[00:04:58] 1009 | |                  is_check: bool)
[00:04:58] 1154 | |     deps
[00:04:58] 1155 | | }
[00:04:58]      | |_- defined here
[00:04:58] 
[00:04:58] 
[00:04:58] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:58]     --> src/bootstrap/check.rs:155:9
[00:04:58]      |
[00:04:58] 155  | /         run_cargo(builder,
[00:04:58] 156  | |                   &mut cargo,
[00:04:58] 157  | |                   args(builder.kind),
[00:04:58] 158  | |                   &codegen_backend_stamp(builder, compiler, target, backend),
[00:04:58] 159  | |                   true);
[00:04:58]      | 
[00:04:58]     ::: src/bootstrap/compile.rs:1006:1
[00:04:58]      |
[00:04:58]      |
[00:04:58] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:58] 1007 | |                  cargo: &mut Command,
[00:04:58] 1008 | |                  stamp: &Path,
[00:04:58] 1009 | |                  is_check: bool)
[00:04:58] 1154 | |     deps
[00:04:58] 1155 | | }
[00:04:58]      | |_- defined here
[00:04:58] 
[00:04:58] 
[00:04:58] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:58]     --> src/bootstrap/check.rs:193:9
[00:04:58]      |
[00:04:58] 193  | /         run_cargo(builder,
[00:04:58] 194  | |                   &mut cargo,
[00:04:58] 195  | |                   args(builder.kind),
[00:04:58] 196  | |                   &libtest_stamp(builder, compiler, target),
[00:04:58] 197  | |                   true);
[00:04:58]      | 
[00:04:58]     ::: src/bootstrap/compile.rs:1006:1
[00:04:58]      |
[00:04:58]      |
[00:04:58] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:58] 1007 | |                  cargo: &mut Command,
[00:04:58] 1008 | |                  stamp: &Path,
[00:04:58] 1009 | |                  is_check: bool)
[00:04:58] 1154 | |     deps
[00:04:58] 1155 | | }
[00:04:58]      | |_- defined here
[00:04:58] 
[00:04:58] 
[00:04:58] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[00:04:58]     --> src/bootstrap/check.rs:241:9
[00:04:58]      |
[00:04:58] 241  | /         run_cargo(builder,
[00:04:58] 242  | |                   &mut cargo,
[00:04:58] 243  | |                   args(builder.kind),
[00:04:58] 244  | |                   &rustdoc_stamp(builder, compiler, target),
[00:04:58] 245  | |                   true);
[00:04:58]      | 
[00:04:58]     ::: src/bootstrap/compile.rs:1006:1
[00:04:58]      |
[00:04:58]      |
[00:04:58] 1006 | / pub fn run_cargo(builder: &Builder,
[00:04:58] 1007 | |                  cargo: &mut Command,
[00:04:58] 1008 | |                  stamp: &Path,
[00:04:58] 1009 | |                  is_check: bool)
[00:04:58] 1154 | |     deps
[00:04:58] 1155 | | }
[00:04:58]      | |_- defined here
[00:04:58] 
---
travis_time:end:2840d3ef:start=1549965960292107228,finish=1549965960298583470,duration=6476242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0722374c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26505ca8
travis_time:start:26505ca8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:110e72f8
$ dmesg | grep -i kill
