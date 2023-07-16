plain
travis_time:end:02114b1f:start=1545243349707315262,finish=1545243352238225664,duration=2530910402
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:16:05] test actions::hover::test::test_process_docs_racer_returns_extra_slashes ... ok
[01:16:05] test actions::hover::test::test_process_docs_rust_blocks ... ok
[01:16:05] test actions::hover::test::test_format_method ... ok
[01:16:05] test actions::hover::test::test_format_object ... ok
[01:16:05] test actions::hover::test::test_tooltip_std ... ignored
[01:16:05] test actions::hover::test::test_tooltip_std_racer ... ignored
[01:16:05] test actions::test::test_find_word_at_pos ... ok
[01:16:05] test actions::test::file_watch_relevant_files ... ok
[01:16:05] test build::auto_tune_build_wait_no_config ... ok
[01:16:05] test build::cargo::test::test_dedup_flags ... ok
---
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> $DIR/foreign-fn-linkname.rs:21:16
[01:23:32]    |
[01:23:32] 21 |     use libc::{c_char, size_t};
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> $DIR/foreign-fn-linkname.rs:21:24
[01:23:32]    |
[01:23:32] 21 |     use libc::{c_char, size_t};
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> $DIR/foreign-fn-linkname.rs:24:38
[01:23:32]    |
[01:23:32] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> $DIR/foreign-fn-linkname.rs:24:49
[01:23:32]    |
[01:23:32] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error: aborting due to 5 previous errors
---
[01:23:32] +
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +  --> $DIR/foreign-fn-linkname.rs:21:16
[01:23:32] +   |
[01:23:32] +21 |     use libc::{c_char, size_t};
[01:23:32] +   |
[01:23:32] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] +
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +  --> $DIR/foreign-fn-linkname.rs:21:24
[01:23:32] +   |
[01:23:32] +21 |     use libc::{c_char, size_t};
[01:23:32] +   |
[01:23:32] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] +
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +  --> $DIR/foreign-fn-linkname.rs:24:38
[01:23:32] +   |
[01:23:32] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32] +   |
[01:23:32] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] +
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] +  --> $DIR/foreign-fn-linkname.rs:24:49
[01:23:32] +   |
[01:23:32] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32] +   |
[01:23:32] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] +
[01:23:32] +error: aborting due to 5 previous errors
[01:23:32] +error: aborting due to 5 previous errors
[01:23:32] +
[01:23:32] +For more information about this error, try `rustc --explain E0658`.
[01:23:32] +
[01:23:32] 
[01:23:32] The actual stderr differed from the expected stderr.
[01:23:32] Actual stderr saved to /tmp/compiletestBY2mUM/foreign-fn-linkname.stderr
[01:23:32] To update references, run this command from build directory:
[01:23:32] tests/run-pass/update-references.sh '/tmp/compiletestBY2mUM' 'foreign-fn-linkname.rs'
[01:23:32] error: 1 errors occurred comparing output.
[01:23:32] status: exit code: 1
[01:23:32] status: exit code: 1
[01:23:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestBY2mUM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBY2mUM/foreign-fn-linkname.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBY2mUM/foreign-fn-linkname.stage-id.aux" "-A" "unused"
[01:23:32] ------------------------------------------
[01:23:32] 
[01:23:32] ------------------------------------------
[01:23:32] stderr:
---
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> tests/run-pass/foreign-fn-linkname.rs:21:16
[01:23:32]    |
[01:23:32] 21 |     use libc::{c_char, size_t};
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> tests/run-pass/foreign-fn-linkname.rs:21:24
[01:23:32]    |
[01:23:32] 21 |     use libc::{c_char, size_t};
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> tests/run-pass/foreign-fn-linkname.rs:24:38
[01:23:32]    |
[01:23:32] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:23:32]   --> tests/run-pass/foreign-fn-linkname.rs:24:49
[01:23:32]    |
[01:23:32] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:23:32]    |
[01:23:32]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:23:32] 
[01:23:32] error: aborting due to 5 previous errors
---
[01:23:32] +For more information about this error, try `rustc --explain E0080`.
[01:23:32] +
[01:23:32] 
[01:23:32] The actual stderr differed from the expected stderr.
[01:23:32] Actual stderr saved to /tmp/compiletestBY2mUM/function_pointers.stderr
[01:23:32] To update references, run this command from build directory:
[01:23:32] tests/run-pass/update-references.sh '/tmp/compiletestBY2mUM' 'function_pointers.rs'
[01:23:32] error: 1 errors occurred comparing output.
[01:23:32] status: exit code: 1
[01:23:32] status: exit code: 1
[01:23:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestBY2mUM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBY2mUM/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBY2mUM/function_pointers.stage-id.aux" "-A" "unused"
[01:23:32] ------------------------------------------
[01:23:32] 
[01:23:32] ------------------------------------------
[01:23:32] stderr:
---
[01:23:32] +For more information about this error, try `rustc --explain E0080`.
[01:23:32] +
[01:23:32] 
[01:23:32] The actual stderr differed from the expected stderr.
[01:23:32] Actual stderr saved to /tmp/compiletestBY2mUM/mir_coercions.stderr
[01:23:32] To update references, run this command from build directory:
[01:23:32] tests/run-pass/update-references.sh '/tmp/compiletestBY2mUM' 'mir_coercions.rs'
[01:23:32] error: 1 errors occurred comparing output.
[01:23:32] status: exit code: 1
[01:23:32] status: exit code: 1
[01:23:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestBY2mUM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBY2mUM/mir_coercions.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBY2mUM/mir_coercions.stage-id.aux" "-A" "unused"
[01:23:32] ------------------------------------------
[01:23:32] 
[01:23:32] ------------------------------------------
[01:23:32] stderr:
---
[01:23:32] +For more information about this error, try `rustc --explain E0658`.
[01:23:32] +
[01:23:32] 
[01:23:32] The actual stderr differed from the expected stderr.
[01:23:32] Actual stderr saved to /tmp/compiletestBY2mUM/regions-mock-trans.stderr
[01:23:32] To update references, run this command from build directory:
[01:23:32] tests/run-pass/update-references.sh '/tmp/compiletestBY2mUM' 'regions-mock-trans.rs'
[01:23:32] error: 1 errors occurred comparing output.
[01:23:32] status: exit code: 1
[01:23:32] status: exit code: 1
[01:23:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestBY2mUM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBY2mUM/regions-mock-trans.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBY2mUM/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:23:32] ------------------------------------------
[01:23:32] 
[01:23:32] ------------------------------------------
[01:23:32] stderr:
---
[01:23:32] +For more information about this error, try `rustc --explain E0658`.
[01:23:32] +
[01:23:32] 
[01:23:32] The actual stderr differed from the expected stderr.
[01:23:32] Actual stderr saved to /tmp/compiletestBY2mUM/thread-local.stderr
[01:23:32] To update references, run this command from build directory:
[01:23:32] tests/run-pass/update-references.sh '/tmp/compiletestBY2mUM' 'thread-local.rs'
[01:23:32] error: 1 errors occurred comparing output.
[01:23:32] status: exit code: 1
[01:23:32] status: exit code: 1
[01:23:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestBY2mUM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBY2mUM/thread-local.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBY2mUM/thread-local.stage-id.aux" "-A" "unused"
[01:23:32] ------------------------------------------
[01:23:32] 
[01:23:32] ------------------------------------------
[01:23:32] stderr:
---
[01:23:32] Verifying status of clippy-driver...
[01:23:32] Verifying status of miri...
[01:23:32] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:23:32] 
[01:23:32] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:23:32] 
[01:23:32] If you do intend to update 'miri', please check the error messages above and
[01:23:32] commit another update.
[01:23:32] 
[01:23:32] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:23:32] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:23:32] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:09a32d26
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 19:39:33 UTC 2018
---
travis_time:end:0f6cd302:start=1545248374434776326,finish=1545248374439133486,duration=4357160
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29c25639
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bbd9ff9
travis_time:start:1bbd9ff9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0373b470
$ dmesg | grep -i kill
