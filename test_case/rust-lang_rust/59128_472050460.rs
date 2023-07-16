plain
travis_time:end:0867b010:start=1552398582869664035,finish=1552398656429761344,duration=73560097309
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:21] 
[01:20:21] running 120 tests
[01:20:47] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:20:51] .i......iii.i.....ii
[01:20:51] 
[01:20:51]  finished in 30.122
[01:20:51] travis_fold:end:test_debuginfo

---
[01:40:42]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2887:58
[01:40:53]      |
[01:40:53] 2887 |             .push(SearchPath::from_cli_opt("native=abc", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2889:57
[01:40:53]      |
[01:40:53] 2889 |             .push(SearchPath::from_cli_opt("crate=def", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2891:62
[01:40:53]      |
[01:40:53] 2891 |             .push(SearchPath::from_cli_opt("dependency=ghi", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2893:61
[01:40:53]      |
[01:40:53] 2893 |             .push(SearchPath::from_cli_opt("framework=jkl", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2895:55
[01:40:53]      |
[01:40:53] 2895 |             .push(SearchPath::from_cli_opt("all=mno", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2898:58
[01:40:53]      |
[01:40:53] 2898 |             .push(SearchPath::from_cli_opt("native=abc", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2900:62
[01:40:53]      |
[01:40:53] 2900 |             .push(SearchPath::from_cli_opt("dependency=ghi", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2902:57
[01:40:53]      |
[01:40:53] 2902 |             .push(SearchPath::from_cli_opt("crate=def", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2904:61
[01:40:53]      |
[01:40:53] 2904 |             .push(SearchPath::from_cli_opt("framework=jkl", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2906:55
[01:40:53]      |
[01:40:53] 2906 |             .push(SearchPath::from_cli_opt("all=mno", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2909:57
[01:40:53]      |
[01:40:53] 2909 |             .push(SearchPath::from_cli_opt("crate=def", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2911:61
[01:40:53]      |
[01:40:53] 2911 |             .push(SearchPath::from_cli_opt("framework=jkl", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2913:58
[01:40:53]      |
[01:40:53] 2913 |             .push(SearchPath::from_cli_opt("native=abc", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2915:62
[01:40:53]      |
[01:40:53] 2915 |             .push(SearchPath::from_cli_opt("dependency=ghi", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2917:55
[01:40:53]      |
[01:40:53] 2917 |             .push(SearchPath::from_cli_opt("all=mno", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2920:55
[01:40:53]      |
[01:40:53] 2920 |             .push(SearchPath::from_cli_opt("all=mno", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2922:58
[01:40:53]      |
[01:40:53] 2922 |             .push(SearchPath::from_cli_opt("native=abc", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2924:57
[01:40:53]      |
[01:40:53] 2924 |             .push(SearchPath::from_cli_opt("crate=def", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2926:62
[01:40:53]      |
[01:40:53] 2926 |             .push(SearchPath::from_cli_opt("dependency=ghi", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
[01:40:53] 
[01:40:53] error[E0423]: expected function, found struct variant `super::ErrorOutputType::Json`
[01:40:53]     --> src/librustc/session/config.rs:2928:61
[01:40:53]      |
[01:40:53] 2928 |             .push(SearchPath::from_cli_opt("framework=jkl", super::ErrorOutputType::Json(false)));
[01:40:53]      |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `super::ErrorOutputType::Json { /* fields */ }`?
[01:40:53]      |
[01:40:53] 2621 |     use test::OutputFormat::Json;
[01:40:53]      |
[01:40:53] 
---
[01:41:21] 
[01:41:21] To learn more, run the command again with --verbose.
[01:41:21] 
[01:41:21] 
[01:41:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:41:21] 
[01:41:21] 
[01:41:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:21] Build completed unsuccessfully in 0:32:45
[01:41:21] Build completed unsuccessfully in 0:32:45
[01:41:21] make: *** [check] Error 1
[01:41:21] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d178592
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 15:32:27 UTC 2019
---
travis_time:end:183c965d:start=1552404749392326510,finish=1552404749398530865,duration=6204355
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017d5d1a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27123db1
travis_time:start:27123db1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00eea3e0
$ dmesg | grep -i kill
