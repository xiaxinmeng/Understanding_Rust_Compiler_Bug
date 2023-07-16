plain
travis_time:end:0dc35ed0:start=1548708763361245099,finish=1548708764381138808,duration=1019893709
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:43]    Compiling serde_derive v1.0.81
[00:02:44]    Compiling serde_json v1.0.33
[00:02:44]    Compiling toml v0.4.10
[00:02:51]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:52] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `if`
[00:02:52]     --> src/bootstrap/builder.rs:1060:13
[00:02:52]      |
[00:02:52] 1058 |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:02:52]      |                                                                   - expected one of `.`, `;`, `?`, `}`, or an operator here
[00:02:52] 1059 | 
[00:02:52] 1060 |             if let Some(ar) = self.ar(target) {
[00:02:52]      |             ^^ unexpected token
[00:02:55] error[E0308]: mismatched types
[00:02:55]     --> src/bootstrap/builder.rs:1057:13
[00:02:55]      |
[00:02:55] 1057 | /             cargo
[00:02:55] 1057 | /             cargo
[00:02:55] 1058 | |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:02:55]      | |                                                                  ^- help: try adding a semicolon: `;`
[00:02:55]      | |__________________________________________________________________|
[00:02:55]      |                                                                    expected (), found mutable reference
[00:02:55]      = note: expected type `()`
[00:02:55]                 found type `&mut std::process::Command`
[00:02:55] 
[00:02:55] error: aborting due to 2 previous errors
---
[00:02:55] make: *** [prepare] Error 1
[00:02:55] Makefile:70: recipe for target 'prepare' failed
[00:02:56] Command failed. Attempt 2/5:
[00:02:57]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:57] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `if`
[00:02:57]     --> src/bootstrap/builder.rs:1060:13
[00:02:57]      |
[00:02:57] 1058 |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:02:57]      |                                                                   - expected one of `.`, `;`, `?`, `}`, or an operator here
[00:02:57] 1059 | 
[00:02:57] 1060 |             if let Some(ar) = self.ar(target) {
[00:02:57]      |             ^^ unexpected token
[00:03:00] error[E0308]: mismatched types
[00:03:00]     --> src/bootstrap/builder.rs:1057:13
[00:03:00]      |
[00:03:00] 1057 | /             cargo
[00:03:00] 1057 | /             cargo
[00:03:00] 1058 | |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:00]      | |                                                                  ^- help: try adding a semicolon: `;`
[00:03:00]      | |__________________________________________________________________|
[00:03:00]      |                                                                    expected (), found mutable reference
[00:03:00]      = note: expected type `()`
[00:03:00]                 found type `&mut std::process::Command`
[00:03:00] 
[00:03:01] error: aborting due to 2 previous errors
---
[00:03:01] Makefile:70: recipe for target 'prepare' failed
[00:03:01] make: *** [prepare] Error 1
[00:03:03] Command failed. Attempt 3/5:
[00:03:03]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:03] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `if`
[00:03:03]     --> src/bootstrap/builder.rs:1060:13
[00:03:03]      |
[00:03:03] 1058 |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:03]      |                                                                   - expected one of `.`, `;`, `?`, `}`, or an operator here
[00:03:03] 1059 | 
[00:03:03] 1060 |             if let Some(ar) = self.ar(target) {
[00:03:03]      |             ^^ unexpected token
[00:03:07] error[E0308]: mismatched types
[00:03:07]     --> src/bootstrap/builder.rs:1057:13
[00:03:07]      |
[00:03:07] 1057 | /             cargo
[00:03:07] 1057 | /             cargo
[00:03:07] 1058 | |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:07]      | |                                                                  ^- help: try adding a semicolon: `;`
[00:03:07]      | |__________________________________________________________________|
[00:03:07]      |                                                                    expected (), found mutable reference
[00:03:07]      = note: expected type `()`
[00:03:07]                 found type `&mut std::process::Command`
[00:03:07] 
[00:03:07] error: aborting due to 2 previous errors
---
[00:03:07] Makefile:70: recipe for target 'prepare' failed
[00:03:07] make: *** [prepare] Error 1
[00:03:10] Command failed. Attempt 4/5:
[00:03:10]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:11] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `if`
[00:03:11]     --> src/bootstrap/builder.rs:1060:13
[00:03:11]      |
[00:03:11] 1058 |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:11]      |                                                                   - expected one of `.`, `;`, `?`, `}`, or an operator here
[00:03:11] 1059 | 
[00:03:11] 1060 |             if let Some(ar) = self.ar(target) {
[00:03:11]      |             ^^ unexpected token
[00:03:14] error[E0308]: mismatched types
[00:03:14]     --> src/bootstrap/builder.rs:1057:13
[00:03:14]      |
[00:03:14] 1057 | /             cargo
[00:03:14] 1057 | /             cargo
[00:03:14] 1058 | |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:14]      | |                                                                  ^- help: try adding a semicolon: `;`
[00:03:14]      | |__________________________________________________________________|
[00:03:14]      |                                                                    expected (), found mutable reference
[00:03:14]      = note: expected type `()`
[00:03:14]                 found type `&mut std::process::Command`
[00:03:14] 
[00:03:14] error: aborting due to 2 previous errors
---
[00:03:14] make: *** [prepare] Error 1
[00:03:14] Makefile:70: recipe for target 'prepare' failed
[00:03:18] Command failed. Attempt 5/5:
[00:03:19]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:19] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `if`
[00:03:19]     --> src/bootstrap/builder.rs:1060:13
[00:03:19]      |
[00:03:19] 1058 |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:19]      |                                                                   - expected one of `.`, `;`, `?`, `}`, or an operator here
[00:03:19] 1059 | 
[00:03:19] 1060 |             if let Some(ar) = self.ar(target) {
[00:03:19]      |             ^^ unexpected token
[00:03:22] error[E0308]: mismatched types
[00:03:22]     --> src/bootstrap/builder.rs:1057:13
[00:03:22]      |
[00:03:22] 1057 | /             cargo
[00:03:22] 1057 | /             cargo
[00:03:22] 1058 | |                 .env(format!("CFLAGS_{}", target), cflags.clone())
[00:03:22]      | |                                                                  ^- help: try adding a semicolon: `;`
[00:03:22]      | |__________________________________________________________________|
[00:03:22]      |                                                                    expected (), found mutable reference
[00:03:22]      = note: expected type `()`
[00:03:22]                 found type `&mut std::process::Command`
[00:03:22] 
[00:03:23] error: aborting due to 2 previous errors
