plain
travis_time:end:00b58e28:start=1545071310492350282,finish=1545071311828781346,duration=1336431064
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:19:27] test test::test_workspace_symbol ... ok
[01:19:27] 
[01:19:27] failures:
[01:19:27] 
[01:19:27] ---- actions::hover::test::test_tooltip stdout ----
[01:19:27] Error: StringError("save_dir does not exist and could not be created: \"/checkout/src/tools/rls/target/tests/hover/save_data\" (Os { code: 30, kind: Other, message: \"Read-only file system\" })")
[01:19:27] thread 'actions::hover::test::test_tooltip' panicked at 'assertion failed: `(left == right)`
[01:19:27]   left: `1`,
[01:19:27]  right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', src/libtest/lib.rs:341:5
[01:19:27] 
[01:19:27] 
[01:19:27] failures:
[01:19:27]     actions::hover::test::test_tooltip
---
[01:27:05] test [ui] run-pass/zst_variant_drop.rs ... ok
ate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] +  --> $DIR/foreign-fn-linkname.rs:24:38
[01:27:05] +   |
[01:27:05] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:27:05] +   |
[01:27:05] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] +
[01:27:05] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] +error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] +  --> $DIR/foreign-fn-linkname.rs:24:49
[01:27:05] +   |
[01:27:05] +24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:27:05] +   |
[01:27:05] +   = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] +
[01:27:05] +error: aborting due to 5 previous errors
[01:27:05] +error: aborting due to 5 previous errors
[01:27:05] +
[01:27:05] +For more information about this error, try `rustc --explain E0658`.
[01:27:05] +
[01:27:05] 
[01:27:05] The actual stderr differed from the expected stderr.
[01:27:05] Actual stderr saved to /tmp/compiletestAp736P/foreign-fn-linkname.stderr
[01:27:05] To update references, run this command from build directory:
[01:27:05] tests/run-pass/update-references.sh '/tmp/compiletestAp736P' 'foreign-fn-linkname.rs'
[01:27:05] error: 1 errors occurred comparing output.
[01:27:05] status: exit code: 1
[01:27:05] status: exit code: 1
[01:27:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestAp736P" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestAp736P/foreign-fn-linkname.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestAp736P/foreign-fn-linkname.stage-id.aux" "-A" "unused"
[01:27:05] ------------------------------------------
[01:27:05] 
[01:27:05] ------------------------------------------
[01:27:05] stderr:
---
[01:27:05] 
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05]   --> tests/run-pass/foreign-fn-linkname.rs:21:16
[01:27:05]    |
[01:27:05] 21 |     use libc::{c_char, size_t};
[01:27:05]    |
[01:27:05]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] 
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05]   --> tests/run-pass/foreign-fn-linkname.rs:21:24
[01:27:05]    |
[01:27:05] 21 |     use libc::{c_char, size_t};
[01:27:05]    |
[01:27:05]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] 
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05]   --> tests/run-pass/foreign-fn-linkname.rs:24:38
[01:27:05]    |
[01:27:05] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:27:05]    |
[01:27:05]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] 
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:27:05]   --> tests/run-pass/foreign-fn-linkname.rs:24:49
[01:27:05]    |
[01:27:05] 24 |         pub fn my_strlen(str: *const c_char) -> size_t;
[01:27:05]    |
[01:27:05]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:27:05] 
[01:27:05] error: aborting due to 5 previous errors
---
[01:27:05] +For more information about this error, try `rustc --explain E0080`.
[01:27:05] +
[01:27:05] 
[01:27:05] The actual stderr differed from the expected stderr.
[01:27:05] Actual stderr saved to /tmp/compiletestAp736P/function_pointers.stderr
[01:27:05] To update references, run this command from build directory:
[01:27:05] tests/run-pass/update-references.sh '/tmp/compiletestAp736P' 'function_pointers.rs'
[01:27:05] error: 1 errors occurred comparing output.
[01:27:05] status: exit code: 1
[01:27:05] status: exit code: 1
[01:27:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestAp736P" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestAp736P/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestAp736P/function_pointers.stage-id.aux" "-A" "unused"
[01:27:05] ------------------------------------------
[01:27:05] 
[01:27:05] ------------------------------------------
[01:27:05] stderr:
---
[01:27:05] +For more information about this error, try `rustc --explain E0080`.
[01:27:05] +
[01:27:05] 
[01:27:05] The actual stderr differed from the expected stderr.
[01:27:05] Actual stderr saved to /tmp/compiletestAp736P/mir_coercions.stderr
[01:27:05] To update references, run this command from build directory:
[01:27:05] tests/run-pass/update-references.sh '/tmp/compiletestAp736P' 'mir_coercions.rs'
[01:27:05] error: 1 errors occurred comparing output.
[01:27:05] status: exit code: 1
[01:27:05] status: exit code: 1
[01:27:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestAp736P" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestAp736P/mir_coercions.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestAp736P/mir_coercions.stage-id.aux" "-A" "unused"
[01:27:05] ------------------------------------------
[01:27:05] 
[01:27:05] ------------------------------------------
[01:27:05] stderr:
---
[01:27:05] 
[01:27:05] The actual stderr differed from the expected stderr.
[01:27:05] Actual stderr saved to /tmp/compiletestAp736P/regions-mock-trans.stderr
[01:27:05] To update references, run this command from build directory:
[01:27:05] tests/run-pass/update-references.sh '/tmp/compiletestAp736P' 'regions-mock-trans.rs'
[01:27:05] error: 1 errors occurred comparing output.
[01:27:05] status: exit code: 1
[01:27:05] status: exit code: 1
[01:27:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestAp736P" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestAp736P/regions-mock-trans.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestAp736P/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:27:05] ------------------------------------------
[01:27:05] 
[01:27:05] ------------------------------------------
[01:27:05] stderr:
---
[01:27:05] 
[01:27:05] The actual stderr differed from the expected stderr.
[01:27:05] Actual stderr saved to /tmp/compiletestAp736P/thread-local.stderr
[01:27:05] To update references, run this command from build directory:
[01:27:05] tests/run-pass/update-references.sh '/tmp/compiletestAp736P' 'thread-local.rs'
[01:27:05] error: 1 errors occurred comparing output.
[01:27:05] status: exit code: 1
[01:27:05] status: exit code: 1
[01:27:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestAp736P" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestAp736P/thread-local.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestAp736P/thread-local.stage-id.aux" "-A" "unused"
[01:27:05] ------------------------------------------
[01:27:05] 
[01:27:05] ------------------------------------------
[01:27:05] stderr:
---
[01:27:05] Verifying status of rust-by-example...
[01:27:05] Verifying status of rls...
[01:27:05] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:27:05] 
[01:27:05] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:27:05] 
[01:27:05] If you do intend to update 'rls', please check the error messages above and
[01:27:05] commit another update.
[01:27:05] 
[01:27:05] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:27:05] change the submodule at 'src/tools/rls'. You may a-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release
214772 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps
210868 ./src/llvm-emscripten/test
201732 ./obj/build/x86_64-unknown-linux-gnu/stage1
201712 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
201712 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
197988 ./obj/build/bootstrap/debug/deps
187096 ./obj/build/cache
187092 ./obj/build/cache/2018-12-09
174128 ./obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86
160376 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
151964 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
144276 ./obj/build/bootstrap/debug/incremental/bootstrap-2pq9xsgmnli7u
144272 ./obj/build/bootstrap/debug/incremental/bootstrap-2pq9xsgmnli7u/s-f7ok08dnod-eay4kx-2xemh5fg4shd2
143512 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
141136 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
136084 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/release
115356 ./src/llvm/test/CodeGen
---
travis_time:end:124bf538:start=1545076546645490401,finish=1545076546651498023,duration=6007622
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:166d3a34
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3a188824
travis_time:start:3a188824
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1aacc0b2
$ dmesg | grep -i kill
