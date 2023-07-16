plain
travis_time:end:0dccee46:start=1561054847727710243,finish=1561054848531866590,duration=804156347
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:31] 
[01:01:31] running 9 tests
[01:01:31] iiiiiiiii
[01:01:31] 
[01:01:31]  finished in 0.148
[01:01:31] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:47] 
[01:01:47] running 122 tests
[01:02:10] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:02:15] .i.i......iii.i.....ii
[01:02:15] 
[01:02:15]  finished in 28.218
[01:02:15] travis_fold:end:test_debuginfo

---
[01:14:22]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:14:29] error[E0308]: mismatched types
[01:14:29]     --> <::alloc::macros::format macros>:2:1
[01:14:29]      |
[01:14:29] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:29] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:29]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:29]      |   expected reference, found struct `realstd::string::String`
[01:14:29]      | 
[01:14:29]     ::: src/libstd/fs.rs:2175:5
[01:14:29]      |
[01:14:29]      |
[01:14:29] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:29]      |       |                                            |
[01:14:29]      |       |                                            in this macro invocation (#2)
[01:14:29]      |       in this expansion of `error!` (#1)
[01:14:29] 2176 | 
[01:14:29] 2176 | 
[01:14:29] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:29] 2178 | |         match $e {
[01:14:29] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:29] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:29] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:29] 2182 | |         }
[01:14:29] 2183 | |     ) }
[01:14:29]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:29] ...
[01:14:29] ...
[01:14:29] 2231 |           error!(result, "No such file or directory");
[01:14:29]      |
[01:14:29]      = note: expected type `&'static str`
[01:14:29]                 found type `realstd::string::String`
[01:14:29] 
[01:14:29] 
[01:14:29] error[E0308]: mismatched types
[01:14:29]     --> <::alloc::macros::format macros>:2:1
[01:14:29]      |
[01:14:29] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:29] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:29]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:29]      |   expected reference, found struct `realstd::string::String`
[01:14:29]      | 
[01:14:29]     ::: src/libstd/fs.rs:2175:5
[01:14:29]      |
[01:14:29]      |
[01:14:29] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:29]      |       |                                            |
[01:14:29]      |       |                                            in this macro invocation (#2)
[01:14:29]      |       in this expansion of `error!` (#1)
[01:14:29] 2176 | 
[01:14:29] 2176 | 
[01:14:29] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:29] 2178 | |         match $e {
[01:14:29] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:29] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:29] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:29] 2182 | |         }
[01:14:29] 2183 | |     ) }
[01:14:29]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:29] ...
[01:14:29] ...
[01:14:29] 2244 |           error!(result, "No such file or directory");
[01:14:29]      |
[01:14:29]      = note: expected type `&'static str`
[01:14:29]                 found type `realstd::string::String`
[01:14:29] 
[01:14:29] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3111 |           error!(c(&r).create_new(true).open(&tmpdir.join("b")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3112 |           error!(c(&r).create(true).truncate(true).open(&tmpdir.join("b")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3113 |           error!(c(&r).truncate(true).open(&tmpdir.join("b")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3114 |           error!(c(&r).create(true).open(&tmpdir.join("b")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3126 |           error!(c(&a).create(true).truncate(true).open(&tmpdir.join("d")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3127 |           error!(c(&a).truncate(true).open(&tmpdir.join("d")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3133 |           error!(c(&ra).create(true).truncate(true).open(&tmpdir.join("e")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3134 |           error!(c(&ra).truncate(true).open(&tmpdir.join("e")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!` (#3)
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2175:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2175 |       macro_rules! error { ($e:expr, $s:expr) => ( error_contains!($e, $s) ) }
[01:14:30]      |       |                                            |
[01:14:30]      |       |                                            in this macro invocation (#2)
[01:14:30]      |       in this expansion of `error!` (#1)
[01:14:30] 2176 | 
[01:14:30] 2176 | 
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!` (#2)
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3140 |            error!(blank.create(true).open(&tmpdir.join("f")), invalid_options);
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
[01:14:30] 
[01:14:30] error[E0308]: mismatched types
[01:14:30]     --> <::alloc::macros::format macros>:2:1
[01:14:30]      |
[01:14:30] 1    | / ( $ ( $ arg : tt ) * ) => (
[01:14:30] 2    | | $ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )
[01:14:30]      | |_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_- in this expansion of `format!`
[01:14:30]      |   expected reference, found struct `realstd::string::String`
[01:14:30]      | 
[01:14:30]     ::: src/libstd/fs.rs:2177:5
[01:14:30]      |
[01:14:30]      |
[01:14:30] 2177 | /     macro_rules! error_contains { ($e:expr, $s:expr) => (
[01:14:30] 2178 | |         match $e {
[01:14:30] 2179 | |             Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
[01:14:30] 2180 | |             Err(ref err) => assert!(err.to_string().contains($s),
[01:14:30] 2181 | |                                     format!("`{}` did not contain `{}`", err, $s))
[01:14:30] 2182 | |         }
[01:14:30] 2183 | |     ) }
[01:14:30]      | |_______- in this expansion of `error_contains!`
[01:14:30] ...
[01:14:30] ...
[01:14:30] 3218 | /         error_contains!(fs::read_to_string(&tmpdir.join("not-utf8")),
[01:14:30] 3219 | |                         "stream did not contain valid UTF-8");
[01:14:30]      |
[01:14:30]      = note: expected type `&'static str`
[01:14:30]                 found type `realstd::string::String`
[01:14:30] 
---
[01:14:40] 
[01:14:40] To learn more, run the command again with --verbose.
[01:14:40] 
[01:14:40] 
[01:14:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:14:40] 
[01:14:40] 
[01:14:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:40] Build completed unsuccessfully in 1:10:22
---
travis_time:end:2cd6cebc:start=1561059341530039278,finish=1561059341535840034,duration=5800756
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a8e59a3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f316603
travis_time:start:0f316603
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:102dacba
$ dmesg | grep -i kill
