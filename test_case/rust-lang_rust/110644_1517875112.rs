plain

---- [ui] tests/ui/test-attrs/tests-listing-format-json.rs stdout ----
diff of run.stdout:

1 { "type": "suite", "event": "discovery" }
- { "type": "test", "event": "discovered", "name": "a_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 20, "start_col": 4, "end_line": 20, "end_col": 10 }
- { "type": "test", "event": "discovered", "name": "m_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 13, "start_col": 4, "end_line": 13, "end_col": 10 }
- { "type": "test", "event": "discovered", "name": "z_test", "ignore": true, "ignore_message": "not yet implemented", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 17, "start_col": 4, "end_line": 17, "end_col": 10 }
+ { "type": "test", "event": "discovered", "name": "a_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 21, "start_col": 4, "end_line": 21, "end_col": 10 }
+ { "type": "test", "event": "discovered", "name": "m_test", "ignore": false, "ignore_message": "", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 14, "start_col": 4, "end_line": 14, "end_col": 10 }
+ { "type": "test", "event": "discovered", "name": "z_test", "ignore": true, "ignore_message": "not yet implemented", "source_path": "$DIR/tests-listing-format-json.rs", "start_line": 18, "start_col": 4, "end_line": 18, "end_col": 10 }
5 { "type": "suite", "event": "completed", "tests": 3, "benchmarks": 0, "total": 3, "ignored": 1 }


The actual run.stdout differed from the expected run.stdout.
The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json/tests-listing-format-json.run.stdout
error: 1 errors occurred comparing run output.
error: 1 errors occurred comparing run output.
failed to decode compiler output as json: line: { "type": "suite", "event": "discovery" }
output: { "type": "suite", "event": "discovery" }
{ "type": "test", "event": "discovered", "name": "a_test", "ignore": false, "ignore_message": "", "source_path": "fake-test-src-base/test-attrs/tests-listing-format-json.rs", "start_line": 21, "start_col": 4, "end_line": 21, "end_col": 10 }
{ "type": "test", "event": "discovered", "name": "m_test", "ignore": false, "ignore_message": "", "source_path": "fake-test-src-base/test-attrs/tests-listing-format-json.rs", "start_line": 14, "start_col": 4, "end_line": 14, "end_col": 10 }
{ "type": "test", "event": "discovered", "name": "z_test", "ignore": true, "ignore_message": "not yet implemented", "source_path": "fake-test-src-base/test-attrs/tests-listing-format-json.rs", "start_line": 18, "start_col": 4, "end_line": 18, "end_col": 10 }
{ "type": "suite", "event": "completed", "tests": 3, "benchmarks": 0, "total": 3, "ignored": 1 }
thread '[ui] tests/ui/test-attrs/tests-listing-format-json.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:132:21

failures:
    [ui] tests/ui/test-attrs/tests-listing-format-json.rs

