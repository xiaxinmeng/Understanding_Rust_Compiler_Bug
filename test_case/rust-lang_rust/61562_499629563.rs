console
[misdreavus@tonberry display-as]$ cargo +stable test --doc                                                                                    [17/1140]
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
   Doc-tests display-as

running 8 tests
test src/lib.rs -  (line 113) ... ok
test src/lib.rs -  (line 149) ... ignored
test src/lib.rs -  (line 100) ... ok
test src/lib.rs -  (line 127) ... ok
test src/html.rs - display_floats_as (line 78) ... ok
test src/lib.rs -  (line 58) ... ok
test src/lib.rs -  (line 72) ... ok
test src/lib.rs -  (line 85) ... ok

test result: ok. 7 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

[misdreavus@tonberry display-as]$ cargo +local test --doc
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
   Doc-tests display-as

running 12 tests                                                                                                                                       test src/lib.rs -  (line 104) ... ok
test src/lib.rs -  (line 156) ... ignored                                                                                                              test src/lib.rs -  (line 178) ... ignored
test src/lib.rs -  (line 183) ... ignored
test src/lib.rs -  (line 188) ... ignored
test src/lib.rs -  (line 133) ... ok
test src/lib.rs -  (line 118) ... ok
test src/html.rs - display_floats_as (line 78) ... ok
test src/lib.rs -  (line 74) ... ok
test src/lib.rs -  (line 59) ... ok
test src/lib.rs -  (line 88) ... ok
test src/lib.rs -  (line 193) ... FAILED

failures:

---- src/lib.rs -  (line 193) stdout ----
thread 'src/lib.rs -  (line 193)' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"<title>PH365: Templates</title>\n<html>\n  <ul><li><span class=\"student\">Name: David</span>\n\n</li><li><span class=\"student\">Name: Joel<
/span>\n\n</li></ul>\n</html>\n\n\n"`,
 right: `"<title>PH365: Templates</title>\n<html>\n  <ul>\n\n  // This is buggy:  I want to iterate, but it fails!\n  for s in self.students.iter() {\n
    \"<li>\" s \"</li>\"\n  }\n\n  </ul>\n</html>\n\n\n"`', src/lib.rs:18:1
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

', src/librustdoc/test.rs:342:17
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    src/lib.rs -  (line 193)

test result: FAILED. 7 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--doc'
