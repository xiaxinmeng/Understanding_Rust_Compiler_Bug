
task 'tests::test_finish_once' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `1`)', run.rs:475
FAILED
test tests::test_finish_twice ... task 'tests::test_finish_twice' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `1`)', run.rs:490
FAILED
test tests::test_finish_with_output_once ... task 'tests::test_finish_with_output_once' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `0`)', run.rs:520
FAILED
test tests::test_finish_with_output_twice ... task 'tests::test_finish_with_output_twice' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `0`)', run.rs:566
FAILED
test tests::test_inherit_env ... ok
test tests::test_keep_current_working_dir ... ok
test tests::test_pipes ... ignored
test tests::test_process_output_error ... task 'tests::test_process_output_error' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `255`)', run.rs:408
FAILED
test tests::test_process_output_output ... task 'tests::test_process_output_output' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `0`)', run.rs:382
FAILED
test tests::test_process_status ... task 'tests::test_process_status' failed at 'assertion failed: `(left == right) && (right == left)` (left: `1081820432`, right: `1`)', run.rs:355
FAILED

failures:
    tests::test_finish_once
    tests::test_finish_twice
    tests::test_finish_with_output_once
    tests::test_finish_with_output_twice
    tests::test_process_output_error
    tests::test_process_output_output
    tests::test_process_status

