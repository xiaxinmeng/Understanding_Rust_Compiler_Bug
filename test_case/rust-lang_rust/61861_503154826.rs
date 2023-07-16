plain
travis_time:end:1871f300:start=1560860419255345912,finish=1560860421379529406,duration=2124183494
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:41:11] 
[01:41:11] failures:
[01:41:11] 
[01:41:11] ---- client_completion_suggests_arguments_in_statements stdout ----
[01:41:11] Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"textDocument": Object({"colorProvider": Null, "completion": Object({"completionItem": Object({"snippetSupport": Bool(true)})})}), "window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir"), "rootUri": Null})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library"), "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("test cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("test cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:41:11] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String(""), "message": String("expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `~`\n\nexpected one of 7 possible tokens here"), "range": Object({"end": Object({"character": Number(42), "line": Number(3)}), "start": Object({"character": Number(41), "line": Number(3)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir/library/tests/test.rs")})})
[01:41:11] Sending: Object({"id": Number(100), "jsonrpc": String("2.0"), "method": String("textDocument/completion"), "params": Object({"context": Object({"triggerCharacter": String("f"), "triggerKind": Number(2)}), "position": Object({"character": Number(41), "line": Number(3)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir/library/tests/test.rs")})})})
[01:41:11] Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([])})
[01:41:11] thread 'client_completion_suggests_arguments_in_statements' panicked at 'Racer autocompletion failed', src/libcore/option.rs:1034:5
[01:41:11] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:41:11] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
---
[01:44:25] test config::test::test_print_docs_include_unstable ... ok
[01:44:25] test config::test::test_config_used_to_toml ... ok
[01:44:25] test config::test::test_dump_default_config ... ok
[01:44:25] test expr::test::test_last_line_offsetted ... ok
[01:44:25] test formatting::newline_style::tests::applies_unix_newlines_to_string_with_unix_and_windows_newlines ... ok
[01:44:25] test config::test::test_was_set ... ok
[01:44:25] test formatting::newline_style::tests::applying_unix_newlines_changes_nothing_for_unix_newlines ... ok
[01:44:25] test formatting::newline_style::tests::applies_unix_newlines ... ok
[01:44:25] test formatting::newline_style::tests::applying_windows_newlines_changes_nothing_for_windows_newlines ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_and_applies_unix_newlines ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_and_applies_native_newlines ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_unix_newlines ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_and_applies_windows_newlines ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_windows_newlines ... ok
[01:44:25] test formatting::newline_style::tests::falls_back_to_native_newlines_if_no_newlines_are_found ... ok
[01:44:25] test formatting::newline_style::tests::auto_detects_windows_newlines_with_multibyte_char_on_first_line ... ok
[01:44:25] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_unix_newlines_to_str_with_windows_newlines ... ok
[01:44:25] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_windows_newlines_to_str_with_unix_newlines ... ok
[01:44:25] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_windows_newlines_to_str_with_windows_newlines ... ok
[01:44:25] test formatting::newline_style::tests::applies_windows_newlines_to_string_with_unix_and_windows_newlines ... ok
[01:44:25] test ignore_path::test::test_ignore_path_set ... ok
[01:44:25] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_unix_newlines_to_str_with_unix_newlines ... ok
[01:44:25] test imports::test::test_use_tree_normalize ... ok
[01:44:25] test issues::find_unnumbered_issue ... ok
[01:44:25] test issues::find_issue ... ok
[01:44:25] test imports::test::test_use_tree_merge ... ok
---
[01:44:25] 
[01:44:25]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/cargo_fmt-6fbf438e3bac2340
[01:44:25] 
[01:44:25] running 11 tests
[01:44:25] test cargo_fmt_tests::default_options ... ok
[01:44:25] test cargo_fmt_tests::empty_packages_3 ... ok
[01:44:25] test cargo_fmt_tests::empty_packages_2 ... ok
[01:44:25] test cargo_fmt_tests::empty_packages_1 ... ok
[01:44:25] test cargo_fmt_tests::good_options ... ok
[01:44:25] test cargo_fmt_tests::mandatory_separator ... ok
[01:44:25] test cargo_fmt_tests::empty_packages_4 ... ok
[01:44:25] test cargo_fmt_tests::multiple_packages_one_by_one ... ok
[01:44:25] test cargo_fmt_tests::multiple_packages_grouped ... ok
[01:44:25] test cargo_fmt_tests::unexpected_option ... ok
[01:44:25] test cargo_fmt_tests::unexpected_flag ... ok
[01:44:25] test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:25] 
[01:44:25]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/git_rustfmt-5c9d961f63afac54
[01:44:25] 
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/assume_bug.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'assume_bug.rs'
[01:46:44] thread '[ui] run-pass/assume_bug.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:44] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/assume_bug.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/assume_bug.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] -args
[01:46:44] -
[01:46:44] 
[01:46:44] The actual stdout differed from the expected stdout.
[01:46:44] Actual stdout saved to /tmp/compiletestodjt05/args.stdout
[01:46:44] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:44]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:44]      |
[01:46:44] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/args.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'args.rs'
[01:46:44] error: 2 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/args.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/args.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/associated-const.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'associated-const.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/associated-const.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/associated-const.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +
[01:46:44] thread '[ui] run-pass/arrays.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/arrays.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'arrays.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/arrays.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/arrays.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +
[01:46:44] thread '[ui] run-pass/atomic-access-bool.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/atomic-access-bool.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'atomic-access-bool.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/atomic-access-bool.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/atomic-access-bool.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/async-fn.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'async-fn.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/async-fn.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/bad_substs.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'bad_substs.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/bad_substs.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/bad_substs.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/atomic-compare_exchange.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'atomic-compare_exchange.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/atomic-compare_exchange.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/atomic-compare_exchange.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/bools.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'bools.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/bools.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/bools.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +
[01:46:44] thread '[ui] run-pass/binops.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/binops.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'binops.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/binops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/binops.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] -foo #1 = Foo(1337)
[01:46:44] -
[01:46:44] 
[01:46:44] The actual stdout differed from the expected stdout.
[01:46:44] Actual stdout saved to /tmp/compiletestodjt05/box-pair-to-vec.stdout
[01:46:44] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:44]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:44]      |
[01:46:44] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:44] +
[01:46:44] test [ui] run-pass/binops.rs ... FAILED
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/box-pair-to-vec.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'box-pair-to-vec.rs'
[01:46:44] error: 2 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/box-pair-to-vec.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/box-pair-to-vec.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/box_box_trait.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'box_box_trait.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/box_box_trait.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/box_box_trait.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/btreemap.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'btreemap.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/btreemap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/btreemap.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/call_drop_on_array_elements.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_on_array_elements.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_on_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
[01:46:44] stdout:
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
---
[01:46:44] +For more information about this error, try `rustc --explain E0080`.
[01:46:44] +
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/c_enums.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'c_enums.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/c_enums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/c_enums.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:44] +
[01:46:44] thread '[ui] run-pass/call_drop_on_fat_ptr_array_elements.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:44] 
[01:46:44] The actual stderr differed from the expected stderr.
[01:46:44] Actual stderr saved to /tmp/compiletestodjt05/call_drop_on_fat_ptr_array_elements.stderr
[01:46:44] To update references, run this command from build directory:
[01:46:44] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_on_fat_ptr_array_elements.rs'
[01:46:44] error: 1 errors occurred comparing output.
[01:46:44] status: exit code: 1
[01:46:44] status: exit code: 1
[01:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_on_fat_ptr_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
[01:46:44] ------------------------------------------
[01:46:44] 
[01:46:44] ------------------------------------------
[01:46:44] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/call_drop_through_trait_object.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_through_trait_object.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_through_trait_object.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/call_drop_on_zst_array_elements.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_on_zst_array_elements.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_on_zst_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +
[01:46:45] thread '[ui] run-pass/call_drop_through_owned_slice.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/call_drop_through_owned_slice.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_through_owned_slice.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_through_owned_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/call_drop_through_trait_object_rc.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'call_drop_through_trait_object_rc.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/call_drop_through_trait_object_rc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/cast_fn_ptr.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'cast_fn_ptr.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/cast_fn_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/cast_fn_ptr.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/calloc.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'calloc.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/calloc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/calloc.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/calls.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'calls.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/calls.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/calls.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +
[01:46:45] thread '[ui] run-pass/cast-rfc0401-vtable-kinds.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/cast-rfc0401-vtable-kinds.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'cast-rfc0401-vtable-kinds.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/cast-rfc0401-vtable-kinds.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +
[01:46:45] thread '[ui] run-pass/cast_fn_ptr_unsafe.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/cast_fn_ptr_unsafe.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'cast_fn_ptr_unsafe.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/cast_fn_ptr_unsafe.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] -1
[01:46:45] -
[01:46:45] 
[01:46:45] The actual stdout differed from the expected stdout.
[01:46:45] Actual stdout saved to /tmp/compiletestodjt05/catch.stdout
[01:46:45] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:45]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:45]      |
[01:46:45] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/catch.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'catch.rs'
[01:46:45] error: 2 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/catch.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/catch.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/closure-drop.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'closure-drop.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/closure-drop.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/closure-drop.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/char.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'char.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/char.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/char.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/closure-field-ty.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'closure-field-ty.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/closure-field-ty.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/closure-field-ty.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/const-vec-of-fns.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'const-vec-of-fns.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/const-vec-of-fns.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/const-vec-of-fns.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/constants.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'constants.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/constants.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/constants.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/closures.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'closures.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/closures.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/closures.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/drop_empty_slice.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'drop_empty_slice.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/drop_empty_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/drop_empty_slice.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/dst-irrefutable-bind.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'dst-irrefutable-bind.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/dst-irrefutable-bind.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/deriving-associated-types.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'deriving-associated-types.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/deriving-associated-types.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/deriving-associated-types.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:45] +For more information about this error, try `rustc --explain E0080`.
[01:46:45] +
[01:46:45] 
[01:46:45] The actual stderr differed from the expected stderr.
[01:46:45] Actual stderr saved to /tmp/compiletestodjt05/dst-field-align.stderr
[01:46:45] To update references, run this command from build directory:
[01:46:45] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'dst-field-align.rs'
[01:46:45] error: 1 errors occurred comparing output.
[01:46:45] status: exit code: 1
[01:46:45] status: exit code: 1
[01:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/dst-field-align.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/dst-field-align.stage-id.aux" "-A" "unused"
[01:46:45] ------------------------------------------
[01:46:45] 
[01:46:45] ------------------------------------------
[01:46:45] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/dst-raw.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'dst-raw.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/dst-raw.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/dst-raw.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/enum-nullable-const-null-with-fields.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'enum-nullable-const-null-with-fields.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/enum-nullable-const-null-with-fields.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/dst-struct-sole.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'dst-struct-sole.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/dst-struct-sole.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/dst-struct-sole.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/exit.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'exit.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/exit.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/exit.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/env.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'env.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/env.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/env.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/dst-struct.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'dst-struct.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/dst-struct.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/dst-struct.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/enums.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'enums.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/enums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/enums.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/extern_types.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'extern_types.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/extern_types.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/extern_types.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/float_fast_math.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'float_fast_math.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/float_fast_math.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/float_fast_math.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/foreign-fn-linkname.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'foreign-fn-linkname.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/foreign-fn-linkname.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/foreign-fn-linkname.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/floats.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'floats.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/floats.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/floats.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] -hello00000
[01:46:46] -
[01:46:46] 
[01:46:46] The actual stdout differed from the expected stdout.
[01:46:46] Actual stdout saved to /tmp/compiletestodjt05/format.stdout
[01:46:46] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:46]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:46]      |
[01:46:46] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/format.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'format.rs'
[01:46:46] error: 2 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/format.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/format.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/from_utf8.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'from_utf8.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/from_utf8.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/from_utf8.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/function_pointers.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'function_pointers.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/function_pointers.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/hashmap.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'hashmap.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletestodjt05/hashmap.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/heap.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'heap.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/heap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/heap.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/generator.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'generator.rs'
[01:46:46] error: 1 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/generator.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/generator.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:46] -Hello, world!
[01:46:46] -
[01:46:46] 
[01:46:46] The actual stdout differed from the expected stdout.
[01:46:46] Actual stdout saved to /tmp/compiletestodjt05/hello.stdout
[01:46:46] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:46]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:46]      |
[01:46:46] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:46] +For more information about this error, try `rustc --explain E0080`.
[01:46:46] +
[01:46:46] 
[01:46:46] The actual stderr differed from the expected stderr.
[01:46:46] Actual stderr saved to /tmp/compiletestodjt05/hello.stderr
[01:46:46] To update references, run this command from build directory:
[01:46:46] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'hello.rs'
[01:46:46] error: 2 errors occurred comparing output.
[01:46:46] status: exit code: 1
[01:46:46] status: exit code: 1
[01:46:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/hello.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/hello.stage-id.aux" "-A" "unused"
[01:46:46] ------------------------------------------
[01:46:46] 
[01:46:46] ------------------------------------------
[01:46:46] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/heap_allocator.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'heap_allocator.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap_allocator.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/heap_allocator.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/heap_allocator.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/intrinsics.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'intrinsics.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/intrinsics.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/intrinsics.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/intrinsics-math.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'intrinsics-math.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/intrinsics-math.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/intrinsics-math.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-15063.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-15063.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-15063.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-15063.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/ints.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ints.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] thread '[ui] run-pass/ints.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:47] thread '[ui] run-pass/ints.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ints.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ints.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/integer-ops.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'integer-ops.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/integer-ops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/integer-ops.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-15080.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-15080.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-15080.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-15080.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-17877.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-17877.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-17877.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-17877.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-15523-big.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-15523-big.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-15523-big.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-15523-big.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-20575.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-20575.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-20575.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-20575.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:47] +For more information about this error, try `rustc --explain E0080`.
[01:46:47] +
[01:46:47] 
[01:46:47] The actual stderr differed from the expected stderr.
[01:46:47] Actual stderr saved to /tmp/compiletestodjt05/issue-26709.stderr
[01:46:47] To update references, run this command from build directory:
[01:46:47] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-26709.rs'
[01:46:47] error: 1 errors occurred comparing output.
[01:46:47] status: exit code: 1
[01:46:47] status: exit code: 1
[01:46:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-26709.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-26709.stage-id.aux" "-A" "unused"
[01:46:47] ------------------------------------------
[01:46:47] 
[01:46:47] ------------------------------------------
[01:46:47] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-23261.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-23261.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-23261.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-23261.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-27901.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-27901.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-27901.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-27901.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-30530.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-30530.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-30530.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-30530.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-29746.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-29746.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-29746.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-29746.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-31267-additional.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-31267-additional.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-31267-additional.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-31267-additional.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/intrinsics-integer.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'intrinsics-integer.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/intrinsics-integer.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/intrinsics-integer.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +
[01:46:48] thread '[ui] run-pass/issue-33387.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-33387.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-33387.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-33387.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-33387.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-34571.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-34571.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-34571.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-34571.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-35815.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-35815.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-35815.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-35815.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] -S { s: 5 }
[01:46:48] -
[01:46:48] 
[01:46:48] The actual stdout differed from the expected stdout.
[01:46:48] Actual stdout saved to /tmp/compiletestodjt05/issue-3794.stdout
[01:46:48] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:46:48]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:46:48]      |
[01:46:48] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-3794.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-3794.rs'
[01:46:48] error: 2 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-3794.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-3794.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-53728.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-53728.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-53728.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-53728.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-36278-prefix-nesting.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-36278-prefix-nesting.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-36278-prefix-nesting.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-5917.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-5917.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-5917.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-5917.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/issue-miri-184.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'issue-miri-184.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/issue-miri-184.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/issue-miri-184.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/last-use-in-cap-clause.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'last-use-in-cap-clause.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/last-use-in-cap-clause.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/iter.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'iter.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/iter.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/iter.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/linked-list.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'linked-list.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/linked-list.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/linked-list.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/main_fn.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'main_fn.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/main_fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/main_fn.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/loops.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'loops.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/loops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/loops.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:48] +For more information about this error, try `rustc --explain E0080`.
[01:46:48] +
[01:46:48] 
[01:46:48] The actual stderr differed from the expected stderr.
[01:46:48] Actual stderr saved to /tmp/compiletestodjt05/many_shr_bor.stderr
[01:46:48] To update references, run this command from build directory:
[01:46:48] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'many_shr_bor.rs'
[01:46:48] error: 1 errors occurred comparing output.
[01:46:48] status: exit code: 1
[01:46:48] status: exit code: 1
[01:46:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/many_shr_bor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/many_shr_bor.stage-id.aux" "-A" "unused"
[01:46:48] ------------------------------------------
[01:46:48] 
[01:46:48] ------------------------------------------
[01:46:48] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/loop-break-value.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'loop-break-value.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/loop-break-value.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/loop-break-value.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/match_slice.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'match_slice.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/match_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/match_slice.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/memchr.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'memchr.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/memchr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/memchr.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/miri-issue-133.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'miri-issue-133.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/miri-issue-133.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/miri-issue-133.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/mir_fat_ptr.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'mir_fat_ptr.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/mir_fat_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/mir_fat_ptr.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/mir_coercions.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'mir_coercions.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/mir_coercions.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/mir_coercions.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/move-arg-2-unique.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'move-arg-2-unique.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/move-arg-2-unique.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/move-arg-2-unique.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/move-undef-primval.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'move-undef-primval.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/move-undef-primval.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/move-undef-primval.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/multi_arg_closure.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'multi_arg_closure.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/multi_arg_closure.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/multi_arg_closure.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +
[01:46:49] thread '[ui] run-pass/move-arg-3-unique.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/move-arg-3-unique.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'move-arg-3-unique.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/move-arg-3-unique.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/move-arg-3-unique.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/negative_discriminant.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'negative_discriminant.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/negative_discriminant.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/negative_discriminant.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/observed_local_mut.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'observed_local_mut.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/observed_local_mut.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestodjt05/observed_local_mut.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/non_capture_closure_to_fn_ptr.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'non_capture_closure_to_fn_ptr.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/non_capture_closure_to_fn_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/option_box_transmute_ptr.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'option_box_transmute_ptr.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/option_box_transmute_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/option_eq.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'option_eq.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/option_eq.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/option_eq.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/overloaded-calls-simple.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'overloaded-calls-simple.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/overloaded-calls-simple.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/overloaded-calls-simple.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/packed_static.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'packed_static.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/packed_static.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/packed_static.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:49] +For more information about this error, try `rustc --explain E0080`.
[01:46:49] +
[01:46:49] 
[01:46:49] The actual stderr differed from the expected stderr.
[01:46:49] Actual stderr saved to /tmp/compiletestodjt05/packed_struct.stderr
[01:46:49] To update references, run this command from build directory:
[01:46:49] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'packed_struct.rs'
[01:46:49] error: 1 errors occurred comparing output.
[01:46:49] status: exit code: 1
[01:46:49] status: exit code: 1
[01:46:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/packed_struct.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/packed_struct.stage-id.aux" "-A" "unused"
[01:46:49] ------------------------------------------
[01:46:49] 
[01:46:49] ------------------------------------------
[01:46:49] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/pointers.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'pointers.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/pointers.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ptr_arith_offset.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ptr_arith_offset.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ptr_arith_offset.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ptr_arith_offset.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/products.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'products.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/products.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/products.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ptr_arith_offset_overflow.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ptr_arith_offset_overflow.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ptr_arith_offset_overflow.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ptr_int_casts.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ptr_int_casts.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ptr_int_casts.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ptr_int_casts.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ptr_offset.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ptr_offset.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ptr_offset.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ptr_offset.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ptr_int_ops.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ptr_int_ops.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ptr_int_ops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/ptr_int_ops.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/raw.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'raw.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/raw.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/raw.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/recursive_static.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'recursive_static.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/recursive_static.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/recursive_static.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +
[01:46:50] thread '[ui] run-pass/ref-invalid-ptr.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/ref-invalid-ptr.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'ref-invalid-ptr.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/ref-invalid-ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestodjt05/ref-invalid-ptr.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/realloc.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'realloc.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/realloc.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/realloc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/realloc.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/rc.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'rc.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/rc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/rc.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/refcell.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'refcell.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/refcell.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/refcell.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/regions-lifetime-nonfree-late-bound.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'regions-lifetime-nonfree-late-bound.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/regions-lifetime-nonfree-late-bound.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/regions-mock-trans.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'regions-mock-trans.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/regions-mock-trans.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/rfc1623.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'rfc1623.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/rfc1623.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/rfc1623.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/sendable-class.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'sendable-class.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/sendable-class.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/sendable-class.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] +For more information about this error, try `rustc --explain E0080`.
[01:46:50] +
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/rust-lang-org.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'rust-lang-org.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/rust-lang-org.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/rust-lang-org.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] 
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/send-is-not-static-par-for.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'send-is-not-static-par-for.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/send-is-not-static-par-for.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:50] The actual stderr differed from the expected stderr.
[01:46:50] Actual stderr saved to /tmp/compiletestodjt05/simd-intrinsic-generic-elements.stderr
[01:46:50] To update references, run this command from build directory:
[01:46:50] thread '[ui] run-pass/simd-intrinsic-generic-elements.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:50] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'simd-intrinsic-generic-elements.rs'
[01:46:50] error: 1 errors occurred comparing output.
[01:46:50] status: exit code: 1
[01:46:50] status: exit code: 1
[01:46:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/simd-intrinsic-generic-elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
[01:46:50] ------------------------------------------
[01:46:50] 
[01:46:50] ------------------------------------------
[01:46:50] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/small_enum_size_bug.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'small_enum_size_bug.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/small_enum_size_bug.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/small_enum_size_bug.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/specialization.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'specialization.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/specialization.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/specialization.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/stacked-borrows/2phase.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'stacked-borrows/2phase.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/stacked-borrows/2phase.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/stacked-borrows/interior_mutability.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'stacked-borrows/interior_mutability.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/stacked-borrows/interior_mutability.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/slices.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'slices.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/slices.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/slices.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/static_memory_modification.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'static_memory_modification.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/static_memory_modification.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/static_memory_modification.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/stacked-borrows/stacked-borrows.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'stacked-borrows/stacked-borrows.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] thread '[ui] run-pass/stacked-borrows/stacked-borrows.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/stacked-borrows/stacked-borrows.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/static_mut.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'static_mut.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] thread '[ui] run-pass/static_mut.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/static_mut.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/static_mut.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/subslice_array.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'subslice_array.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/subslice_array.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/subslice_array.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/strings.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'strings.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/strings.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/strings.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/sums.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'sums.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/sums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/sums.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/tag-align-dyn-u64.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'tag-align-dyn-u64.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/tag-align-dyn-u64.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/threads.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'threads.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/threads.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/threads.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/threads.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/too-large-primval-write-problem.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'too-large-primval-write-problem.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/too-large-primval-write-problem.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +
[01:46:51] thread '[ui] run-pass/thread-local.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/thread-local.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'thread-local.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/thread-local.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/thread-local.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/trivial.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'trivial.rs'
[01:46:51] 
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/trivial.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/trivial.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/transmute_fat.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'transmute_fat.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/transmute_fat.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/transmute_fat.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/traits.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'traits.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/traits.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/traits.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:51] +For more information about this error, try `rustc --explain E0080`.
[01:46:51] +
[01:46:51] 
[01:46:51] The actual stderr differed from the expected stderr.
[01:46:51] Actual stderr saved to /tmp/compiletestodjt05/try-operator-custom.stderr
[01:46:51] To update references, run this command from build directory:
[01:46:51] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'try-operator-custom.rs'
[01:46:51] error: 1 errors occurred comparing output.
[01:46:51] status: exit code: 1
[01:46:51] status: exit code: 1
[01:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/try-operator-custom.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/try-operator-custom.stage-id.aux" "-A" "unused"
[01:46:51] ------------------------------------------
[01:46:51] 
[01:46:51] ------------------------------------------
[01:46:51] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/tuple_like_enum_variant_constructor.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'tuple_like_enum_variant_constructor.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/tuple_like_enum_variant_constructor_pointer_opt.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/tuple_like_struct_constructor.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'tuple_like_struct_constructor.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/tuple_like_struct_constructor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/union-overwrite.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'union-overwrite.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/union-overwrite.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/union-overwrite.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/union.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'union.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/union.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/union.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/unique-send.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'unique-send.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unique-send.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/unique-send.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/unique-send.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/unops.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'unops.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/unops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/unops.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/validation_lifetime_resolution.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'validation_lifetime_resolution.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/validation_lifetime_resolution.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/unsized-tuple-impls.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'unsized-tuple-impls.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/unsized-tuple-impls.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/unsized-tuple-impls.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] thread '[ui] run-pass/u128.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/u128.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'u128.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/u128.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/u128.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] -[2, 2] Iter([2, 2], [])
[01:46:52] -Iter([], [])
[01:46:52] -
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'vec-matching-fold.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/vec-matching-fold.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/vec-matching-fold.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/vecdeque.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'vecdeque.rs'
[01:46:52] error: 2 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/vecdeque.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/vecdeque.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +For more information about this error, try `rustc --explain E0080`.
[01:46:52] +
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/volatile.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'volatile.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/volatile.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/volatile.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/vecs.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'vecs.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/vecs.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/vecs.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +
[01:46:52] thread '[ui] run-pass/zero-sized-binary-heap-push.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/zero-sized-binary-heap-push.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'zero-sized-binary-heap-push.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/zero-sized-binary-heap-push.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:52] +
[01:46:52] thread '[ui] run-pass/without-validation.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:52] 
[01:46:52] The actual stderr differed from the expected stderr.
[01:46:52] Actual stderr saved to /tmp/compiletestodjt05/without-validation.stderr
[01:46:52] To update references, run this command from build directory:
[01:46:52] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'without-validation.rs'
[01:46:52] error: 1 errors occurred comparing output.
[01:46:52] status: exit code: 1
[01:46:52] status: exit code: 1
[01:46:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/without-validation.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestodjt05/without-validation.stage-id.aux" "-A" "unused"
[01:46:52] ------------------------------------------
[01:46:52] 
[01:46:52] ------------------------------------------
[01:46:52] stderr:
---
[01:46:53] +
[01:46:53] thread '[ui] run-pass/write-bytes.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:46:53] 
[01:46:53] The actual stderr differed from the expected stderr.
[01:46:53] Actual stderr saved to /tmp/compiletestodjt05/write-bytes.stderr
[01:46:53] To update references, run this command from build directory:
[01:46:53] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'write-bytes.rs'
[01:46:53] error: 1 errors occurred comparing output.
[01:46:53] status: exit code: 1
[01:46:53] status: exit code: 1
[01:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/write-bytes.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/write-bytes.stage-id.aux" "-A" "unused"
[01:46:53] ------------------------------------------
[01:46:53] 
[01:46:53] ------------------------------------------
[01:46:53] stderr:
---
[01:46:53] +For more information about this error, try `rustc --explain E0080`.
[01:46:53] +
[01:46:53] 
[01:46:53] The actual stderr differed from the expected stderr.
[01:46:53] Actual stderr saved to /tmp/compiletestodjt05/zst.stderr
[01:46:53] To update references, run this command from build directory:
[01:46:53] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'zst.rs'
[01:46:53] error: 1 errors occurred comparing output.
[01:46:53] status: exit code: 1
[01:46:53] status: exit code: 1
[01:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/zst.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/zst.stage-id.aux" "-A" "unused"
[01:46:53] ------------------------------------------
[01:46:53] 
[01:46:53] ------------------------------------------
[01:46:53] stderr:
---
[01:46:53] +For more information about this error, try `rustc --explain E0080`.
[01:46:53] +
[01:46:53] 
[01:46:53] The actual stderr differed from the expected stderr.
[01:46:53] Actual stderr saved to /tmp/compiletestodjt05/zst_box.stderr
[01:46:53] To update references, run this command from build directory:
[01:46:53] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'zst_box.rs'
[01:46:53] error: 1 errors occurred comparing output.
[01:46:53] status: exit code: 1
[01:46:53] status: exit code: 1
[01:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/zst_box.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/zst_box.stage-id.aux" "-A" "unused"
[01:46:53] ------------------------------------------
[01:46:53] 
[01:46:53] ------------------------------------------
[01:46:53] stderr:
---
[01:46:53] +For more information about this error, try `rustc --explain E0080`.
[01:46:53] +
[01:46:53] 
[01:46:53] The actual stderr differed from the expected stderr.
[01:46:53] Actual stderr saved to /tmp/compiletestodjt05/zst_variant_drop.stderr
[01:46:53] To update references, run this command from build directory:
[01:46:53] tests/run-pass/update-references.sh '/tmp/compiletestodjt05' 'zst_variant_drop.rs'
[01:46:53] error: 1 errors occurred comparing output.
[01:46:53] status: exit code: 1
[01:46:53] status: exit code: 1
[01:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestodjt05" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestodjt05/zst_variant_drop.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestodjt05/zst_variant_drop.stage-id.aux" "-A" "unused"
[01:46:53] ------------------------------------------
[01:46:53] 
[01:46:53] ------------------------------------------
[01:46:53] stderr:
---
[01:46:53] Verifying status of edition-guide...
[01:46:53] Verifying status of rls...
[01:46:53] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:46:53] 
[01:46:53] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:46:53] 
[01:46:53] If you do intend to update 'rls', please check the error messages above and
[01:46:53] commit another update.
[01:46:53] 
[01:46:53] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:46:53] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:46:53] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0017325c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jun 18 14:07:25 UTC 2019
---
travis_time:end:0eb569a0:start=1560866846558032951,finish=1560866846563199290,duration=5166339
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:078c922e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25fdc6e2
travis_time:start:25fdc6e2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0097ee56
$ dmesg | grep -i kill
