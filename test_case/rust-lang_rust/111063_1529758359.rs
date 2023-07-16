plain

---- io::util::tests::copy_specializes_bufwriter stdout ----
thread 'io::util::tests::copy_specializes_bufwriter' panicked at 'assertion failed: `(left == right)`
  left: `8192`,
 right: `16384`: expected a large buffer to be provided to the reader', library/std/src/io/util/tests.rs:61:5

failures:
    io::util::tests::copy_specializes_bufwriter


test result: FAILED. 936 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.29s

error: test failed, to rerun pass `-p std --lib`
