plain

---- [ui] tests/ui/test-attrs/tests-listing-format-json.rs stdout ----
diff of run.stdout:

- { "type": "suite", "event": "discovery" }
- { "type": "test", "event": "discovered", "name": "a_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 20, "start_col": 4, "end_line": 20, "end_col": 10 }
- { "type": "test", "event": "discovered", "name": "m_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 13, "start_col": 4, "end_line": 13, "end_col": 10 }
- { "type": "test", "event": "discovered", "name": "z_test", "ignore": true, "ignore_message": "not yet implemented", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 17, "start_col": 4, "end_line": 17, "end_col": 10 }
- { "type": "suite", "event": "completed", "tests": 3, "benchmarks": 0, "total": 3, "ignored": 1 }


The actual run.stdout differed from the expected run.stdout.
The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json/tests-listing-format-json.run.stdout
normalized run.stderr:
error: the option `Z` is only accepted on the nightly compiler


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json/tests-listing-format-json.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json" && RUST_TEST_THREADS="8" "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json/a.js" "--list" "--format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
error: the option `Z` is only accepted on the nightly compiler



failures:
