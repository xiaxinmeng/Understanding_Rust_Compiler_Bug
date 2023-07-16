plain
travis_time:end:079a927d:start=1552133961660549537,finish=1552133964385607011,duration=2725057474
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:16:37] .................................................................................................... 3700/5440
[01:16:41] ..................................................ii................................................ 3800/5440
[01:16:43] ....................................................................i............................... 3900/5440
[01:16:46] .................................................................................................... 4000/5440
[01:16:48] ..........................i.F....................................................................... 4100/5440
[01:17:05] .................................................................................................... 4300/5440
[01:17:09] .................................................................................................... 4400/5440
[01:17:12] .................................................................................................... 4500/5440
[01:17:16] .....................................................................................i.............. 4600/5440
---
[01:17:52] 
[01:17:52] ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
[01:17:52] diff of stderr:
[01:17:52] 
[01:17:52] 1 error[E0583]: file not found for module `not_a_real_file`
[01:17:52] 3    |
[01:17:52] 3    |
[01:17:52] - LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
[01:17:52] + LL | mod not_a_real_file;
[01:17:52] 6    |
[01:17:52] 6    |
[01:17:52] 7    = help: name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory "$DIR"
[01:17:52] 
[01:17:52] The actual stderr differed from the expected stderr.
[01:17:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/mod_file_not_exist.stderr
[01:17:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/mod_file_not_exist.stderr
[01:17:52] To update references, rerun the tests and pass the `--bless` flag
[01:17:52] To only update this specific test, also pass `--test-args parser/mod_file_not_exist.rs`
[01:17:52] error: 1 errors occurred comparing output.
[01:17:52] status: exit code: 1
[01:17:52] status: exit code: 1
[01:17:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_not_exist.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/auxiliary" "-A" "unused"
[01:17:52] ------------------------------------------
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] stderr:
[01:17:52] stderr:
[01:17:52] ------------------------------------------
[01:17:52] {"message":"file not found for module `not_a_real_file`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n