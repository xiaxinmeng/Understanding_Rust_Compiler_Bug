plain
travis_time:end:088407a2:start=1560847705572684141,finish=1560847706407736373,duration=835052232
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:42:37] 
[01:42:37] failures:
[01:42:37] 
[01:42:37] ---- client_completion_suggests_arguments_in_statements stdout ----
[01:42:37] Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"textDocument": Object({"colorProvider": Null, "completion": Object({"completionItem": Object({"snippetSupport": Bool(true)})})}), "window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir"), "rootUri": Null})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library"), "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("test cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("test cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:42:37] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String(""), "message": String("expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `~`\n\nexpected one of 7 possible tokens here"), "range": Object({"end": Object({"character": Number(42), "line": Number(3)}), "start": Object({"character": Number(41), "line": Number(3)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir/library/tests/test.rs")})})
[01:42:37] Sending: Object({"id": Number(100), "jsonrpc": String("2.0"), "method": String("textDocument/completion"), "params": Object({"context": Object({"triggerCharacter": String("f"), "triggerKind": Number(2)}), "position": Object({"character": Number(41), "line": Number(3)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/ws_with_test_dir/library/tests/test.rs")})})})
[01:42:37] Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([])})
[01:42:37] thread 'client_completion_suggests_arguments_in_statements' panicked at 'Racer autocompletion failed', src/libcore/option.rs:1034:5
[01:42:37] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:37] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
---
[01:45:50] test config::test::test_print_docs_include_unstable ... ok
[01:45:50] test expr::test::test_last_line_offsetted ... ok
[01:45:50] test formatting::newline_style::tests::applies_unix_newlines ... ok
[01:45:50] test config::test::test_was_set ... ok
[01:45:50] test formatting::newline_style::tests::applies_windows_newlines_to_string_with_unix_and_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::applies_unix_newlines_to_string_with_unix_and_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::applying_unix_newlines_changes_nothing_for_unix_newlines ... ok
[01:45:50] test formatting::newline_style::tests::applying_windows_newlines_changes_nothing_for_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_and_applies_native_newlines ... ok
[01:45:50] test comment::test::format_doc_comments ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_and_applies_unix_newlines ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_and_applies_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_unix_newlines ... ok
[01:45:50] test formatting::newline_style::tests::falls_back_to_native_newlines_if_no_newlines_are_found ... ok
[01:45:50] test formatting::newline_style::tests::auto_detects_windows_newlines_with_multibyte_char_on_first_line ... ok
[01:45:50] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_unix_newlines_to_str_with_windows_newlines ... ok
[01:45:50] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_unix_newlines_to_str_with_unix_newlines ... ok
[01:45:50] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_windows_newlines_to_str_with_unix_newlines ... ok
[01:45:50] test formatting::newline_style::tests::keeps_carriage_returns_when_applying_windows_newlines_to_str_with_windows_newlines ... ok
[01:45:50] test imports::test::test_use_tree_normalize ... ok
[01:45:50] test imports::test::test_use_tree_flatten ... ok
[01:45:50] test issues::find_issue ... ok
[01:45:50] test issues::find_unnumbered_issue ... ok
---
[01:45:51] 
[01:45:51]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/cargo_fmt-6fbf438e3bac2340
[01:45:51] 
[01:45:51] running 11 tests
[01:45:51] test cargo_fmt_tests::default_options ... ok
[01:45:51] test cargo_fmt_tests::empty_packages_3 ... ok
[01:45:51] test cargo_fmt_tests::empty_packages_2 ... ok
[01:45:51] test cargo_fmt_tests::empty_packages_1 ... ok
[01:45:51] test cargo_fmt_tests::empty_packages_4 ... ok
[01:45:51] test cargo_fmt_tests::good_options ... ok
[01:45:51] test cargo_fmt_tests::mandatory_separator ... ok
[01:45:51] test cargo_fmt_tests::multiple_packages_one_by_one ... ok
[01:45:51] test cargo_fmt_tests::unexpected_flag ... ok
[01:45:51] test cargo_fmt_tests::multiple_packages_grouped ... ok
[01:45:51] test cargo_fmt_tests::unexpected_option ... ok
[01:45:51] test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:51] 
[01:45:51]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/git_rustfmt-5c9d961f63afac54
[01:45:51] 
---
[01:48:08] +For more information about this error, try `rustc --explain E0080`.
[01:48:08] +
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/assume_bug.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'assume_bug.rs'
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/assume_bug.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/assume_bug.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] -args
[01:48:08] -
[01:48:08] 
[01:48:08] The actual stdout differed from the expected stdout.
[01:48:08] Actual stdout saved to /tmp/compiletestyqVhhj/args.stdout
[01:48:08] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:08]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:08]      |
[01:48:08] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:08] +For more information about this error, try `rustc --explain E0080`.
[01:48:08] +
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/args.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'args.rs'
[01:48:08] error: 2 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/args.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/args.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] +
[01:48:08] thread '[ui] run-pass/associated-const.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/associated-const.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'associated-const.rs'
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/associated-const.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/associated-const.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] +For more information about this error, try `rustc --explain E0080`.
[01:48:08] +
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/arrays.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'arrays.rs'
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/arrays.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/arrays.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] +For more information about this error, try `rustc --explain E0080`.
[01:48:08] +
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/async-fn.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'async-fn.rs'
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/async-fn.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] +For more information about this error, try `rustc --explain E0080`.
[01:48:08] +
[01:48:08] 
[01:48:08] The actual stderr differed from the expected stderr.
[01:48:08] Actual stderr saved to /tmp/compiletestyqVhhj/atomic-access-bool.stderr
[01:48:08] To update references, run this command from build directory:
[01:48:08] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'atomic-access-bool.rs'
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 1
[01:48:08] status: exit code: 1
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/atomic-access-bool.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/atomic-access-bool.stage-id.aux" "-A" "unused"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/bad_substs.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'bad_substs.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/bad_substs.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/bad_substs.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/atomic-compare_exchange.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'atomic-compare_exchange.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/atomic-compare_exchange.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/atomic-compare_exchange.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/bools.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'bools.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/bools.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/bools.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/box_box_trait.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'box_box_trait.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/box_box_trait.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/box_box_trait.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] -foo #1 = Foo(1337)
[01:48:09] -
[01:48:09] 
[01:48:09] The actual stdout differed from the expected stdout.
[01:48:09] Actual stdout saved to /tmp/compiletestyqVhhj/box-pair-to-vec.stdout
[01:48:09] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:09]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:09]      |
[01:48:09] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/box-pair-to-vec.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'box-pair-to-vec.rs'
[01:48:09] error: 2 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/box-pair-to-vec.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/box-pair-to-vec.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +
[01:48:09] thread '[ui] run-pass/binops.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/binops.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'binops.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/binops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/binops.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_on_array_elements.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_on_array_elements.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_on_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/btreemap.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'btreemap.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/btreemap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/btreemap.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/c_enums.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'c_enums.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/c_enums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/c_enums.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_on_fat_ptr_array_elements.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_on_fat_ptr_array_elements.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_on_fat_ptr_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_through_trait_object.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_through_trait_object.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_through_trait_object.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_through_trait_object_rc.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_through_trait_object_rc.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_through_trait_object_rc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_through_owned_slice.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_through_owned_slice.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_through_owned_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/call_drop_on_zst_array_elements.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'call_drop_on_zst_array_elements.rs'
[01:48:09] thread '[ui] run-pass/call_drop_on_zst_array_elements.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/call_drop_on_zst_array_elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/cast_fn_ptr.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'cast_fn_ptr.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/cast_fn_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/cast_fn_ptr.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +
[01:48:09] thread '[ui] run-pass/cast-rfc0401-vtable-kinds.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/cast-rfc0401-vtable-kinds.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'cast-rfc0401-vtable-kinds.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/cast-rfc0401-vtable-kinds.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] thread '[ui] run-pass/calls.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:09] thread '[ui] run-pass/calloc.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/calls.stderr
[01:48:09] +     = note: inside call to `std::vec::Vec::<u8>::extend_from_slice` at /checkout/src/liballoc/slice.rs:159:9
[01:48:09] +     = note: inside call to `std::slice::hack::to_vec::<u8>` at /checkout/src/liballoc/slice.rs:379:9
[01:48:09] +     = note: inside call to `std::slice::<impl [u8]>::to_vec` at /checkout/src/liballoc/slice.rs:647:9
[01:48:09] +     = note: inside call to `std::slice::<impl std::borrow::ToOwned for [u8]>::to_owned` at /checkout/src/liballoc/str.rs:206:46
[01:48:09] +     = note: inside call to `std::slice::<impl std::borrow::ToOwned for [u8]>::to_owned` at /checkout/src/liballoc/str.rs:206:46
[01:48:09] +     = note: inside call to `std::str::<impl std::borrow::ToOwned for str>::to_owned` at /checkout/src/libstd/rt.rs:40:39
[01:48:09] +     = note: inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:64:5
[01:48:09] +     = note: inside call to `std::rt::lang_start::<()>`
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'calls.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/calls.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/calls.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/calloc.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'calloc.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/calloc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/calloc.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/cast_fn_ptr_unsafe.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'cast_fn_ptr_unsafe.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/cast_fn_ptr_unsafe.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] -1
[01:48:09] -
[01:48:09] 
[01:48:09] The actual stdout differed from the expected stdout.
[01:48:09] Actual stdout saved to /tmp/compiletestyqVhhj/catch.stdout
[01:48:09] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:09]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:09]      |
[01:48:09] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/catch.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'catch.rs'
[01:48:09] error: 2 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/catch.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/catch.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/closure-drop.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'closure-drop.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/closure-drop.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/closure-drop.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:09] +For more information about this error, try `rustc --explain E0080`.
[01:48:09] +
[01:48:09] 
[01:48:09] The actual stderr differed from the expected stderr.
[01:48:09] Actual stderr saved to /tmp/compiletestyqVhhj/char.stderr
[01:48:09] To update references, run this command from build directory:
[01:48:09] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'char.rs'
[01:48:09] error: 1 errors occurred comparing output.
[01:48:09] status: exit code: 1
[01:48:09] status: exit code: 1
[01:48:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/char.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/char.stage-id.aux" "-A" "unused"
[01:48:09] ------------------------------------------
[01:48:09] 
[01:48:09] ------------------------------------------
[01:48:09] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/closure-field-ty.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'closure-field-ty.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/closure-field-ty.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/closure-field-ty.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/const-vec-of-fns.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'const-vec-of-fns.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/const-vec-of-fns.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/const-vec-of-fns.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/constants.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'constants.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/constants.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/constants.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/closures.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'closures.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/closures.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/closures.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/drop_empty_slice.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'drop_empty_slice.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/drop_empty_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/drop_empty_slice.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/dst-irrefutable-bind.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'dst-irrefutable-bind.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/dst-irrefutable-bind.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/dst-field-align.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'dst-field-align.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/dst-field-align.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/dst-field-align.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/deriving-associated-types.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'deriving-associated-types.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/deriving-associated-types.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/deriving-associated-types.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/dst-raw.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'dst-raw.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/dst-raw.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/dst-raw.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/enum-nullable-const-null-with-fields.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'enum-nullable-const-null-with-fields.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/enum-nullable-const-null-with-fields.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/dst-struct-sole.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'dst-struct-sole.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/dst-struct-sole.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/dst-struct-sole.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/env.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'env.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/env.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/env.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/enums.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'enums.rs'
[01:48:10] 
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/enums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/enums.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/dst-struct.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'dst-struct.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/dst-struct.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/dst-struct.stage-id.aux" "-A" "unused"
[01:48:10] stdout:
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
---
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] test [ui] run-pass/enums.rs ... FAILED
[01:48:10] test [ui] run-pass/dst-struct.rs ... FAILED
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/exit.stderr
[01:48:10] thread '[ui] run-pass/exit.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:10] thread '[ui] run-pass/exit.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'exit.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/exit.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/exit.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/extern_types.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'extern_types.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/extern_types.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/extern_types.stage-id.aux" "-A" "unused"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] ------------------------------------------
[01:48:10] stderr:
---
[01:48:10] +For more information about this error, try `rustc --explain E0080`.
[01:48:10] +
[01:48:10] 
[01:48:10] The actual stderr differed from the expected stderr.
[01:48:10] Actual stderr saved to /tmp/compiletestyqVhhj/foreign-fn-linkname.stderr
[01:48:10] To update references, run this command from build directory:
[01:48:10] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'foreign-fn-linkname.rs'
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 1
[01:48:10] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/foreign-fn-linkname.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/foreign-fn-linkname.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/float_fast_math.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'float_fast_math.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/float_fast_math.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/float_fast_math.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/floats.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'floats.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/floats.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/floats.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] -hello00000
[01:48:11] -
[01:48:11] 
[01:48:11] The actual stdout differed from the expected stdout.
[01:48:11] Actual stdout saved to /tmp/compiletestyqVhhj/format.stdout
[01:48:11] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:11]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:11]      |
[01:48:11] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/format.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'format.rs'
[01:48:11] error: 2 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/format.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/format.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/from_utf8.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'from_utf8.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/from_utf8.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/from_utf8.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/function_pointers.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'function_pointers.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/function_pointers.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/hashmap.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'hashmap.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletestyqVhhj/hashmap.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/generator.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'generator.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/generator.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/generator.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/heap.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'heap.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/heap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/heap.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] -Hello, world!
[01:48:11] -
[01:48:11] 
[01:48:11] The actual stdout differed from the expected stdout.
[01:48:11] Actual stdout saved to /tmp/compiletestyqVhhj/hello.stdout
[01:48:11] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:11]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:11]      |
[01:48:11] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/hello.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'hello.rs'
[01:48:11] error: 2 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/hello.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/hello.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/heap_allocator.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'heap_allocator.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap_allocator.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/heap_allocator.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/heap_allocator.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/intrinsics.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'intrinsics.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/intrinsics.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/intrinsics.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/intrinsics-math.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'intrinsics-math.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/intrinsics-math.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/intrinsics-math.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:11] +For more information about this error, try `rustc --explain E0080`.
[01:48:11] +
[01:48:11] 
[01:48:11] The actual stderr differed from the expected stderr.
[01:48:11] Actual stderr saved to /tmp/compiletestyqVhhj/issue-15063.stderr
[01:48:11] To update references, run this command from build directory:
[01:48:11] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-15063.rs'
[01:48:11] error: 1 errors occurred comparing output.
[01:48:11] status: exit code: 1
[01:48:11] status: exit code: 1
[01:48:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-15063.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-15063.stage-id.aux" "-A" "unused"
[01:48:11] ------------------------------------------
[01:48:11] 
[01:48:11] ------------------------------------------
[01:48:11] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/ints.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ints.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ints.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ints.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/integer-ops.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'integer-ops.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/integer-ops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/integer-ops.stage-id.aux" "-A" "unused"
[01:48:12] stdout:
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-15080.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-15080.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-15080.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-15080.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +
[01:48:12] thread '[ui] run-pass/issue-17877.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-17877.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-17877.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-17877.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-17877.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-15523-big.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-15523-big.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-15523-big.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-15523-big.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-20575.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-20575.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-20575.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-20575.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-26709.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-26709.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-26709.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-26709.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] thread 'thread '[ui] run-pass/issue-23261.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:12] [ui] run-pass/issue-26709.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-23261.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-23261.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-23261.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-23261.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-27901.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-27901.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-27901.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-27901.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-30530.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-30530.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-30530.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-30530.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-29746.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-29746.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-29746.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-29746.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-31267-additional.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-31267-additional.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-31267-additional.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-31267-additional.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-33387.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-33387.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-33387.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-33387.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] test [ui] run-pass/issue-33387.rs ... FAILED
[01:48:12] 
[01:48:12] thread '[ui] run-pass/intrinsics-integer.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/intrinsics-integer.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'intrinsics-integer.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/intrinsics-integer.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/intrinsics-integer.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-34571.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-34571.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-34571.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-34571.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-35815.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-35815.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-35815.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-35815.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] -S { s: 5 }
[01:48:12] -
[01:48:12] 
[01:48:12] The actual stdout differed from the expected stdout.
[01:48:12] Actual stdout saved to /tmp/compiletestyqVhhj/issue-3794.stdout
[01:48:12] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:12]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:12]      |
[01:48:12] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:12] +For more information about this error, try `rustc --explain E0080`.
[01:48:12] +
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-3794.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-3794.rs'
[01:48:12] error: 2 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-3794.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-3794.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:12] 
[01:48:12] test [ui] run-pass/issue-3794.rs ... FAILED
[01:48:12] 
[01:48:12] The actual stderr differed from the expected stderr.
[01:48:12] Actual stderr saved to /tmp/compiletestyqVhhj/issue-36278-prefix-nesting.stderr
[01:48:12] To update references, run this command from build directory:
[01:48:12] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-36278-prefix-nesting.rs'
[01:48:12] error: 1 errors occurred comparing output.
[01:48:12] status: exit code: 1
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-36278-prefix-nesting.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
[01:48:12] ------------------------------------------
[01:48:12] 
[01:48:12] ------------------------------------------
[01:48:12] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/issue-53728.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-53728.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-53728.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-53728.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/issue-5917.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-5917.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-5917.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-5917.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/issue-miri-184.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'issue-miri-184.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/issue-miri-184.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/issue-miri-184.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +
[01:48:13] thread '[ui] run-pass/last-use-in-cap-clause.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/last-use-in-cap-clause.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'last-use-in-cap-clause.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/last-use-in-cap-clause.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +
[01:48:13] thread '[ui] run-pass/iter.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/iter.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'iter.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/iter.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/iter.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/linked-list.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'linked-list.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/linked-list.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/linked-list.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/main_fn.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'main_fn.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/main_fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/main_fn.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +
[01:48:13] thread '[ui] run-pass/loops.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/loops.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'loops.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/loops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/loops.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/loop-break-value.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'loop-break-value.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/loop-break-value.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/loop-break-value.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/many_shr_bor.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'many_shr_bor.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/many_shr_bor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/many_shr_bor.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/match_slice.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'match_slice.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/match_slice.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/match_slice.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/memchr.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'memchr.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/memchr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/memchr.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/mir_fat_ptr.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'mir_fat_ptr.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/mir_fat_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/mir_fat_ptr.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/miri-issue-133.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'miri-issue-133.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/miri-issue-133.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/miri-issue-133.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/mir_coercions.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'mir_coercions.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/mir_coercions.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/mir_coercions.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/move-undef-primval.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'move-undef-primval.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/move-undef-primval.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/move-undef-primval.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/move-arg-2-unique.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'move-arg-2-unique.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/move-arg-2-unique.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/move-arg-2-unique.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/multi_arg_closure.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'multi_arg_closure.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/multi_arg_closure.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/multi_arg_closure.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:13] +For more information about this error, try `rustc --explain E0080`.
[01:48:13] +
[01:48:13] 
[01:48:13] The actual stderr differed from the expected stderr.
[01:48:13] Actual stderr saved to /tmp/compiletestyqVhhj/move-arg-3-unique.stderr
[01:48:13] To update references, run this command from build directory:
[01:48:13] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'move-arg-3-unique.rs'
[01:48:13] error: 1 errors occurred comparing output.
[01:48:13] status: exit code: 1
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/move-arg-3-unique.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/move-arg-3-unique.stage-id.aux" "-A" "unused"
[01:48:13] ------------------------------------------
[01:48:13] 
[01:48:13] ------------------------------------------
[01:48:13] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/negative_discriminant.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'negative_discriminant.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/negative_discriminant.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/negative_discriminant.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/observed_local_mut.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'observed_local_mut.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/observed_local_mut.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestyqVhhj/observed_local_mut.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/option_box_transmute_ptr.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'option_box_transmute_ptr.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/option_box_transmute_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/non_capture_closure_to_fn_ptr.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'non_capture_closure_to_fn_ptr.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/non_capture_closure_to_fn_ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/option_eq.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'option_eq.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/option_eq.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/option_eq.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/overloaded-calls-simple.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'overloaded-calls-simple.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/overloaded-calls-simple.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/overloaded-calls-simple.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/packed_static.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'packed_static.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/packed_static.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/packed_static.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/packed_struct.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'packed_struct.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/packed_struct.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/packed_struct.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ptr_arith_offset.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ptr_arith_offset.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ptr_arith_offset.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ptr_arith_offset.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/pointers.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'pointers.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/pointers.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/products.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'products.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/products.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/products.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ptr_arith_offset_overflow.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ptr_arith_offset_overflow.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ptr_arith_offset_overflow.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ptr_int_casts.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ptr_int_casts.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ptr_int_casts.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ptr_int_casts.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ptr_offset.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ptr_offset.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ptr_offset.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ptr_offset.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ptr_int_ops.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ptr_int_ops.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ptr_int_ops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/ptr_int_ops.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/raw.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'raw.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/raw.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/raw.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/recursive_static.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'recursive_static.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/recursive_static.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/recursive_static.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/realloc.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'realloc.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/realloc.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/realloc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/realloc.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:14] +For more information about this error, try `rustc --explain E0080`.
[01:48:14] +
[01:48:14] 
[01:48:14] The actual stderr differed from the expected stderr.
[01:48:14] Actual stderr saved to /tmp/compiletestyqVhhj/ref-invalid-ptr.stderr
[01:48:14] To update references, run this command from build directory:
[01:48:14] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'ref-invalid-ptr.rs'
[01:48:14] error: 1 errors occurred comparing output.
[01:48:14] status: exit code: 1
[01:48:14] status: exit code: 1
[01:48:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/ref-invalid-ptr.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestyqVhhj/ref-invalid-ptr.stage-id.aux" "-A" "unused"
[01:48:14] ------------------------------------------
[01:48:14] 
[01:48:14] ------------------------------------------
[01:48:14] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/rc.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'rc.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/rc.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/rc.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/refcell.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'refcell.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/refcell.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/refcell.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/regions-lifetime-nonfree-late-bound.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'regions-lifetime-nonfree-late-bound.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/regions-lifetime-nonfree-late-bound.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/regions-mock-trans.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'regions-mock-trans.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/regions-mock-trans.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/rfc1623.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'rfc1623.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/rfc1623.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/rfc1623.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/rust-lang-org.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'rust-lang-org.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/rust-lang-org.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/rust-lang-org.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/send-is-not-static-par-for.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'send-is-not-static-par-for.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/send-is-not-static-par-for.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/sendable-class.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'sendable-class.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/sendable-class.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/sendable-class.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/simd-intrinsic-generic-elements.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'simd-intrinsic-generic-elements.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/simd-intrinsic-generic-elements.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/small_enum_size_bug.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'small_enum_size_bug.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/small_enum_size_bug.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/small_enum_size_bug.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/specialization.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'specialization.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/specialization.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/specialization.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/stacked-borrows/2phase.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'stacked-borrows/2phase.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/stacked-borrows/2phase.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/stacked-borrows/interior_mutability.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'stacked-borrows/interior_mutability.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/stacked-borrows/interior_mutability.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/slices.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'slices.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/slices.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/slices.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/static_memory_modification.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'static_memory_modification.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/static_memory_modification.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/static_memory_modification.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/stacked-borrows/stacked-borrows.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'stacked-borrows/stacked-borrows.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/stacked-borrows/stacked-borrows.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/static_mut.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'static_mut.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/static_mut.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/static_mut.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:15] +For more information about this error, try `rustc --explain E0080`.
[01:48:15] +
[01:48:15] 
[01:48:15] The actual stderr differed from the expected stderr.
[01:48:15] Actual stderr saved to /tmp/compiletestyqVhhj/strings.stderr
[01:48:15] To update references, run this command from build directory:
[01:48:15] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'strings.rs'
[01:48:15] error: 1 errors occurred comparing output.
[01:48:15] status: exit code: 1
[01:48:15] status: exit code: 1
[01:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/strings.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/strings.stage-id.aux" "-A" "unused"
[01:48:15] ------------------------------------------
[01:48:15] 
[01:48:15] ------------------------------------------
[01:48:15] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/subslice_array.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'subslice_array.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/subslice_array.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/subslice_array.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/sums.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'sums.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/sums.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/sums.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/tag-align-dyn-u64.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'tag-align-dyn-u64.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/tag-align-dyn-u64.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/threads.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'threads.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/threads.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/threads.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/threads.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/thread-local.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'thread-local.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/thread-local.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/thread-local.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/too-large-primval-write-problem.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'too-large-primval-write-problem.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/too-large-primval-write-problem.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/traits.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'traits.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/traits.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/traits.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/trivial.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'trivial.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/trivial.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/trivial.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/try-operator-custom.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'try-operator-custom.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/try-operator-custom.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/try-operator-custom.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/transmute_fat.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'transmute_fat.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/transmute_fat.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/transmute_fat.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'tuple_like_enum_variant_constructor.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/tuple_like_struct_constructor.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'tuple_like_struct_constructor.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/tuple_like_struct_constructor.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_pointer_opt.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/union-overwrite.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'union-overwrite.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/union-overwrite.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/union-overwrite.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/union.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'union.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/union.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/union.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/unique-send.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'unique-send.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unique-send.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/unique-send.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/unique-send.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/unops.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'unops.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/unops.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/unops.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:16] +For more information about this error, try `rustc --explain E0080`.
[01:48:16] +
[01:48:16] 
[01:48:16] The actual stderr differed from the expected stderr.
[01:48:16] Actual stderr saved to /tmp/compiletestyqVhhj/unsized-tuple-impls.stderr
[01:48:16] To update references, run this command from build directory:
[01:48:16] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'unsized-tuple-impls.rs'
[01:48:16] error: 1 errors occurred comparing output.
[01:48:16] status: exit code: 1
[01:48:16] status: exit code: 1
[01:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/unsized-tuple-impls.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/unsized-tuple-impls.stage-id.aux" "-A" "unused"
[01:48:16] ------------------------------------------
[01:48:16] 
[01:48:16] ------------------------------------------
[01:48:16] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/validation_lifetime_resolution.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'validation_lifetime_resolution.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/validation_lifetime_resolution.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/u128.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'u128.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/u128.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/u128.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/vec-matching-fold.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'vec-matching-fold.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/vec-matching-fold.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/vec-matching-fold.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/volatile.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'volatile.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/volatile.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/volatile.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] -Iter([], [])
[01:48:17] -
[01:48:17] 
[01:48:17] The actual stdout differed from the expected stdout.
[01:48:17] Actual stdout saved to /tmp/compiletestyqVhhj/vecdeque.stdout
[01:48:17] error[E0080]: Miri evaluation error: unimplemented intrinsic: unchecked_sub
[01:48:17]     --> /checkout/src/libcore/slice/mod.rs:3015:33
[01:48:17]      |
[01:48:17] 3015 |             let diff = unsafe { unchecked_sub($self.end as usize, start as usize) };
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/vecdeque.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'vecdeque.rs'
[01:48:17] error: 2 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/vecdeque.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/vecdeque.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/vecs.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'vecs.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/vecs.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/vecs.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/without-validation.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'without-validation.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/without-validation.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestyqVhhj/without-validation.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/write-bytes.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'write-bytes.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/write-bytes.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/write-bytes.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/zero-sized-binary-heap-push.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'zero-sized-binary-heap-push.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/zero-sized-binary-heap-push.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/zst.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'zst.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/zst.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/zst.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/zst_box.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'zst_box.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/zst_box.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/zst_box.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] +For more information about this error, try `rustc --explain E0080`.
[01:48:17] +
[01:48:17] 
[01:48:17] The actual stderr differed from the expected stderr.
[01:48:17] Actual stderr saved to /tmp/compiletestyqVhhj/zst_variant_drop.stderr
[01:48:17] To update references, run this command from build directory:
[01:48:17] tests/run-pass/update-references.sh '/tmp/compiletestyqVhhj' 'zst_variant_drop.rs'
[01:48:17] error: 1 errors occurred comparing output.
[01:48:17] status: exit code: 1
[01:48:17] status: exit code: 1
[01:48:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestyqVhhj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyqVhhj/zst_variant_drop.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestyqVhhj/zst_variant_drop.stage-id.aux" "-A" "unused"
[01:48:17] ------------------------------------------
[01:48:17] 
[01:48:17] ------------------------------------------
[01:48:17] stderr:
---
[01:48:17] This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...
[01:48:17] Verifying status of clippy-driver...
[01:48:17] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:48:17] 
[01:48:17] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:48:17] 
[01:48:17] If you do intend to update 'clippy-driver', please check the error messages above and
[01:48:17] commit another update.
[01:48:17] 
[01:48:17] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:48:17] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:48:17] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0b80beda
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jun 18 10:36:55 UTC 2019
