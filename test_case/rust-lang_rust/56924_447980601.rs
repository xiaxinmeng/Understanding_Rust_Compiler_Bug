plain
[01:22:24] test test::test_workspace_symbol_duplicates ... ok
[01:22:24] 
[01:22:24] failures:
[01:22:24] 
[01:22:24] ---- actions::hover::test::test_tooltip stdout ----
[01:22:24] Error: StringError("save_dir does not exist and could not be created: \"/checkout/src/tools/rls/target/tests/hover/save_data\" (Os { code: 30, kind: Other, message: \"Read-only file system\" })")
[01:22:24] thread 'actions::hover::test::test_tooltip' panicked at 'assertion failed: `(left == right)`
[01:22:24]   left: `1`,
[01:22:24]  right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', src/libtest/lib.rs:341:5
[01:22:24] 
[01:22:24] 
[01:22:24] failures:
[01:22:24]     actions::hover::test::test_tooltip
---
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> $DIR/foreign-fn-linkname.rs:21:16
[01:30:06]    |
[01:30:06] 21 |     use libc::{c_char, size_t};
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> $DIR/foreign-fn-linkname.rs:21:24
[01:30:06]    |
[01:30:06] 21 |     use libc::{c_char, size_t};
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> $DIR/foreign-fn-linkname.rs:24:38
[01:30:06]    |
[01:30:06] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> $DIR/foreign-fn-linkname.rs:24:49
[01:30:06]    |
[01:30:06] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error: aborting due to 5 previous errors
---
[01:30:06] +
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +  --> $DIR/foreign-fn-linkname.rs:21:16
[01:30:06] +   |
[01:30:06] +21 |     use libc::{c_char, size_t};
[01:30:06] +   |
[01:30:06] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] +
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +  --> $DIR/foreign-fn-linkname.rs:21:24
[01:30:06] +   |
[01:30:06] +21 |     use libc::{c_char, size_t};
[01:30:06] +   |
[01:30:06] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] +
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +  --> $DIR/foreign-fn-linkname.rs:24:38
[01:30:06] +   |
[01:30:06] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06] +   |
[01:30:06] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] +
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] +  --> $DIR/foreign-fn-linkname.rs:24:49
[01:30:06] +   |
[01:30:06] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06] +   |
[01:30:06] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] +
[01:30:06] +error: aborting due to 5 previous errors
[01:30:06] +error: aborting due to 5 previous errors
[01:30:06] +
[01:30:06] +For more information about this error, try `rustc --explain E0658`.
[01:30:06] +
[01:30:06] 
[01:30:06] The actual stderr differed from the expected stderr.
[01:30:06] Actual stderr saved to /tmp/compiletestwbwIHL/foreign-fn-linkname.stderr
[01:30:06] To update references, run this command from build directory:
[01:30:06] tests/run-pass/update-references.sh '/tmp/compiletestwbwIHL' 'foreign-fn-linkname.rs'
[01:30:06] error: 1 errors occurred comparing output.
[01:30:06] status: exit code: 1
[01:30:06] status: exit code: 1
[01:30:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestwbwIHL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwbwIHL/foreign-fn-linkname.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestwbwIHL/foreign-fn-linkname.stage-id.aux" "-A" "unused"
[01:30:06] ------------------------------------------
[01:30:06] 
[01:30:06] ------------------------------------------
[01:30:06] stderr:
---
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> tests/run-pass/foreign-fn-linkname.rs:21:16
[01:30:06]    |
[01:30:06] 21 |     use libc::{c_char, size_t};
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> tests/run-pass/foreign-fn-linkname.rs:21:24
[01:30:06]    |
[01:30:06] 21 |     use libc::{c_char, size_t};
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> tests/run-pass/foreign-fn-linkname.rs:24:38
[01:30:06]    |
[01:30:06] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:30:06]   --> tests/run-pass/foreign-fn-linkname.rs:24:49
[01:30:06]    |
[01:30:06] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:30:06]    |
[01:30:06]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:30:06] 
[01:30:06] error: aborting due to 5 previous errors
---
[01:30:06] +For more information about this error, try `rustc --explain E0080`.
[01:30:06] +
[01:30:06] 
[01:30:06] The actual stderr differed from the expected stderr.
[01:30:06] Actual stderr saved to /tmp/compiletestwbwIHL/function_pointers.stderr
[01:30:06] To update references, run this command from build directory:
[01:30:06] tests/run-pass/update-references.sh '/tmp/compiletestwbwIHL' 'function_pointers.rs'
[01:30:06] error: 1 errors occurred comparing output.
[01:30:06] status: exit code: 1
[01:30:06] status: exit code: 1
[01:30:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestwbwIHL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwbwIHL/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestwbwIHL/function_pointers.stage-id.aux" "-A" "unused"
[01:30:06] ------------------------------------------
[01:30:06] 
[01:30:06] ------------------------------------------
[01:30:06] stderr:
---
[01:30:06] +For more information about this error, try `rustc --explain E0080`.
[01:30:06] +
[01:30:06] 
[01:30:06] The actual stderr differed from the expected stderr.
[01:30:06] Actual stderr saved to /tmp/compiletestwbwIHL/mir_coercions.stderr
[01:30:06] To update references, run this command from build directory:
[01:30:06] tests/run-pass/update-references.sh '/tmp/compiletestwbwIHL' 'mir_coercions.rs'
[01:30:06] error: 1 errors occurred comparing output.
[01:30:06] status: exit code: 1
[01:30:06] status: exit code: 1
[01:30:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestwbwIHL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwbwIHL/mir_coercions.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestwbwIHL/mir_coercions.stage-id.aux" "-A" "unused"
[01:30:06] ------------------------------------------
[01:30:06] 
[01:30:06] ------------------------------------------
[01:30:06] stderr:
---
[01:30:06] +For more information about this error, try `rustc --explain E0658`.
[01:30:06] +
[01:30:06] 
[01:30:06] The actual stderr differed from the expected stderr.
[01:30:06] Actual stderr saved to /tmp/compiletestwbwIHL/regions-mock-trans.stderr
[01:30:06] To update references, run this command from build directory:
[01:30:06] tests/run-pass/update-references.sh '/tmp/compiletestwbwIHL' 'regions-mock-trans.rs'
[01:30:06] error: 1 errors occurred comparing output.
[01:30:06] status: exit code: 1
[01:30:06] status: exit code: 1
[01:30:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestwbwIHL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwbwIHL/regions-mock-trans.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestwbwIHL/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:30:06] ------------------------------------------
[01:30:06] 
[01:30:06] ------------------------------------------
[01:30:06] stderr:
---
[01:30:06] +For more information about this error, try `rustc --explain E0658`.
[01:30:06] +
[01:30:06] 
[01:30:06] The actual stderr differed from the expected stderr.
[01:30:06] Actual stderr saved to /tmp/compiletestwbwIHL/thread-local.stderr
[01:30:06] To update references, run this command from build directory:
[01:30:06] tests/run-pass/update-references.sh '/tmp/compiletestwbwIHL' 'thread-local.rs'
[01:30:06] error: 1 errors occurred comparing output.
[01:30:06] status: exit code: 1
[01:30:06] status: exit code: 1
[01:30:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestwbwIHL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwbwIHL/thread-local.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestwbwIHL/thread-local.stage-id.aux" "-A" "unused"
[01:30:06] ------------------------------------------
[01:30:06] 
[01:30:06] ------------------------------------------
[01:30:06] stderr:
---
[01:30:06] Verifying status of rust-by-example...
[01:30:06] Verifying status of rls...
[01:30:06] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:30:06] 
[01:30:06] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:30:06] 
[01:30:06] If you do intend to update 'rls', please check the error messages above and
[01:30:06] commit another update.
[01:30:06] 
[01:30:06] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:30:06] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:30:06] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:24d124fe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 20:04:13 UTC 2018
---
travis_time:end:0f5bda2c:start=1545077054391806042,finish=1545077054399338157,duration=7532115
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00afb434
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0007e92e
travis_time:start:0007e92e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:009e4534
$ dmesg | grep -i kill
