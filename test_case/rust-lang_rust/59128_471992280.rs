plain
travis_time:end:255143e2:start=1552392589641132664,finish=1552392681630903073,duration=91989770409
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:54:38]    Compiling parking_lot_core v0.4.0
[00:54:42]    Compiling tempfile v3.0.5
[00:54:43]    Compiling parking_lot v0.7.1
[00:54:44]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:54:45] error[E0423]: expected function, found struct variant `ErrorOutputType::Json`
[00:54:45]     |
[00:54:45]     |
[00:54:45] 259 |             Some("json") => ErrorOutputType::Json(false),
[00:54:45]     |                             ^^^^^^^^^^^^^^^^^^^^^ did you mean `ErrorOutputType::Json { /* fields */ }`?
[00:54:45] 
[00:54:45] error[E0423]: expected function, found struct variant `ErrorOutputType::Json`
[00:54:45]     |
[00:54:45]     |
[00:54:45] 260 |             Some("pretty-json") => ErrorOutputType::Json(true),
[00:54:45]     |                                    ^^^^^^^^^^^^^^^^^^^^^ did you mean `ErrorOutputType::Json { /* fields */ }`?
[00:54:45] 
[00:54:45] error[E0532]: expected tuple struct/variant, found struct variant `ErrorOutputType::Json`
[00:54:45]     |
[00:54:45]     |
[00:54:45] 302 |         ErrorOutputType::Json(pretty) => {
[00:54:45]     |         ^^^^^^^^^^^^^^^^^^^^^ did you mean `ErrorOutputType::Json { /* fields */ }`?
[00:54:48] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[00:54:48]    --> src/librustdoc/core.rs:306:17
[00:54:48]     |
[00:54:48] 306 | /                 JsonEmitter::stderr(
[00:54:48] 306 | /                 JsonEmitter::stderr(
[00:54:48] 307 | |                     None,
[00:54:48] 308 | |                     source_map,
[00:54:48] 309 | |                     pretty,
[00:54:48] 310 | |                 ).ui_testing(ui_testing)
[00:54:48] 
[00:54:49] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[00:54:49]    --> src/librustdoc/test.rs:385:23
[00:54:49]     |
[00:54:49]     |
[00:54:49] 385 |         let emitter = EmitterWriter::new(box io::sink(), None, false, false);
[00:54:49] 
[00:54:50] error: aborting due to 5 previous errors
[00:54:50] 
[00:54:50] Some errors occurred: E0061, E0423, E0532.
