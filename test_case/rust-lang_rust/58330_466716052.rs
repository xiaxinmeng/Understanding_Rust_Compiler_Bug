plain
travis_time:end:091c19d0:start=1550966806933647647,finish=1550966809206830415,duration=2273182768
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:30]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:32] error[E0308]: mismatched types
[00:02:32]    --> src/bootstrap/test.rs:653:53
[00:02:32]     |
[00:02:32] 653 |                         if path.extension() == Some("rs") {
[00:02:32]     |                                                     ^^^^ expected struct `std::ffi::OsStr`, found str
[00:02:32]     = note: expected type `&std::ffi::OsStr`
[00:02:32]                found type `&'static str`
[00:02:32] 
[00:02:32] 
[00:02:32] error[E0277]: the trait bound `std::option::Option<&std::ffi::OsStr>: std::convert::AsRef<std::path::Path>` is not satisfied
[00:02:32]    --> src/bootstrap/test.rs:654:43
[00:02:32]     |
[00:02:32] 654 |                             let out = out.join(path.file_stem());
[00:02:32]     |                                           ^^^^ the trait `std::convert::AsRef<std::path::Path>` is not implemented for `std::option::Option<&std::ffi::OsStr>`
[00:02:32] 
[00:02:32] error[E0599]: no method named `to_os_string` found for type `std::option::Option<&std::ffi::OsStr>` in the current scope
[00:02:32]    --> src/bootstrap/test.rs:659:63
[00:02:32]     |
[00:02:32] 659 |                             tests_to_run.push(out.file_stem().to_os_string());
[00:02:32] 
[00:02:34] error: aborting due to 3 previous errors
[00:02:34] 
[00:02:34] Some errors occurred: E0277, E0308, E0599.
---
[00:02:36]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:38] error[E0308]: mismatched types
[00:02:38]    --> src/bootstrap/test.rs:653:53
[00:02:38]     |
[00:02:38] 653 |                         if path.extension() == Some("rs") {
[00:02:38]     |                                                     ^^^^ expected struct `std::ffi::OsStr`, found str
[00:02:38]     = note: expected type `&std::ffi::OsStr`
[00:02:38]                found type `&'static str`
[00:02:38] 
[00:02:38] 
[00:02:38] error[E0277]: the trait bound `std::option::Option<&std::ffi::OsStr>: std::convert::AsRef<std::path::Path>` is not satisfied
[00:02:38]    --> src/bootstrap/test.rs:654:43
[00:02:38]     |
[00:02:38] 654 |                             let out = out.join(path.file_stem());
[00:02:38]     |                                           ^^^^ the trait `std::convert::AsRef<std::path::Path>` is not implemented for `std::option::Option<&std::ffi::OsStr>`
[00:02:38] 
[00:02:38] error[E0599]: no method named `to_os_string` found for type `std::option::Option<&std::ffi::OsStr>` in the current scope
[00:02:38]    --> src/bootstrap/test.rs:659:63
[00:02:38]     |
[00:02:38] 659 |                             tests_to_run.push(out.file_stem().to_os_string());
[00:02:38] 
[00:02:39] error: aborting due to 3 previous errors
[00:02:39] 
[00:02:39] Some errors occurred: E0277, E0308, E0599.
---
[00:02:42]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:44] error[E0308]: mismatched types
[00:02:44]    --> src/bootstrap/test.rs:653:53
[00:02:44]     |
[00:02:44] 653 |                         if path.extension() == Some("rs") {
[00:02:44]     |                                                     ^^^^ expected struct `std::ffi::OsStr`, found str
[00:02:44]     = note: expected type `&std::ffi::OsStr`
[00:02:44]                found type `&'static str`
[00:02:44] 
[00:02:44] 
[00:02:44] error[E0277]: the trait bound `std::option::Option<&std::ffi::OsStr>: std::convert::AsRef<std::path::Path>` is not satisfied
[00:02:44]    --> src/bootstrap/test.rs:654:43
[00:02:44]     |
[00:02:44] 654 |                             let out = out.join(path.file_stem());
[00:02:44]     |                                           ^^^^ the trait `std::convert::AsRef<std::path::Path>` is not implemented for `std::option::Option<&std::ffi::OsStr>`
[00:02:44] 
[00:02:44] error[E0599]: no method named `to_os_string` found for type `std::option::Option<&std::ffi::OsStr>` in the current scope
[00:02:44]    --> src/bootstrap/test.rs:659:63
[00:02:44]     |
[00:02:44] 659 |                             tests_to_run.push(out.file_stem().to_os_string());
[00:02:44] 
[00:02:46] error: aborting due to 3 previous errors
[00:02:46] 
[00:02:46] Some errors occurred: E0277, E0308, E0599.
---
[00:02:49]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:51] error[E0308]: mismatched types
[00:02:51]    --> src/bootstrap/test.rs:653:53
[00:02:51]     |
[00:02:51] 653 |                         if path.extension() == Some("rs") {
[00:02:51]     |                                                     ^^^^ expected struct `std::ffi::OsStr`, found str
[00:02:51]     = note: expected type `&std::ffi::OsStr`
[00:02:51]                found type `&'static str`
[00:02:51] 
[00:02:51] 
[00:02:51] error[E0277]: the trait bound `std::option::Option<&std::ffi::OsStr>: std::convert::AsRef<std::path::Path>` is not satisfied
[00:02:51]    --> src/bootstrap/test.rs:654:43
[00:02:51]     |
[00:02:51] 654 |                             let out = out.join(path.file_stem());
[00:02:51]     |                                           ^^^^ the trait `std::convert::AsRef<std::path::Path>` is not implemented for `std::option::Option<&std::ffi::OsStr>`
[00:02:51] 
[00:02:51] error[E0599]: no method named `to_os_string` found for type `std::option::Option<&std::ffi::OsStr>` in the current scope
[00:02:51]    --> src/bootstrap/test.rs:659:63
[00:02:51]     |
[00:02:51] 659 |                             tests_to_run.push(out.file_stem().to_os_string());
[00:02:51] 
[00:02:53] error: aborting due to 3 previous errors
[00:02:53] 
[00:02:53] Some errors occurred: E0277, E0308, E0599.
---
[00:02:57]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:00] error[E0308]: mismatched types
[00:03:00]    --> src/bootstrap/test.rs:653:53
[00:03:00]     |
[00:03:00] 653 |                         if path.extension() == Some("rs") {
[00:03:00]     |                                                     ^^^^ expected struct `std::ffi::OsStr`, found str
[00:03:00]     = note: expected type `&std::ffi::OsStr`
[00:03:00]                found type `&'static str`
[00:03:00] 
[00:03:00] 
[00:03:00] error[E0277]: the trait bound `std::option::Option<&std::ffi::OsStr>: std::convert::AsRef<std::path::Path>` is not satisfied
[00:03:00]    --> src/bootstrap/test.rs:654:43
[00:03:00]     |
[00:03:00] 654 |                             let out = out.join(path.file_stem());
[00:03:00]     |                                           ^^^^ the trait `std::convert::AsRef<std::path::Path>` is not implemented for `std::option::Option<&std::ffi::OsStr>`
[00:03:00] 
[00:03:00] error[E0599]: no method named `to_os_string` found for type `std::option::Option<&std::ffi::OsStr>` in the current scope
[00:03:00]    --> src/bootstrap/test.rs:659:63
[00:03:00]     |
[00:03:00] 659 |                             tests_to_run.push(out.file_stem().to_os_string());
[00:03:00] 
[00:03:01] error: aborting due to 3 previous errors
[00:03:01] 
[00:03:01] Some errors occurred: E0277, E0308, E0599.
