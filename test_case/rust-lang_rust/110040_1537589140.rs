plain

failures:

---- persist::fs::tests::test_all_except_most_recent stdout ----
thread 'persist::fs::tests::test_all_except_most_recent' panicked at 'assertion failed: `(left == right)`
  left: `UnordSet { inner: {"4", "1", "5", "2", "3"} }`,
 right: `UnordSet { inner: {"4", "1", "2", "3"} }`', compiler/rustc_incremental/src/persist/fs/tests.rs:14:5


failures:
    persist::fs::tests::test_all_except_most_recent
    persist::fs::tests::test_all_except_most_recent

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 671.11µs

error: test failed, to rerun pass `-p rustc_incremental --lib`
