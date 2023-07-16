plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
test test::configuration_snippet::configuration_snippet_tests ... ok
error: `box_syntax` has been removed
 --> tests/source/expr.rs:6:27
  |
6 |     let boxed: Box<i32> = box   5;
  |
help: use `Box::new()` instead
  |
  |
6 |     let boxed: Box<i32> = Box::new(5);

error: `box_syntax` has been removed
   --> tests/source/expr.rs:137:23
    |
    |
137 |         let handler = box DocumentProgressHandler::new(addr, DocumentProgressTask::DOMContentLoaded);
    |
help: use `Box::new()` instead
    |
    |
137 |         let handler = Box::new(DocumentProgressHandler::new(addr, DocumentProgressTask::DOMContentLoaded));

error: `box_syntax` has been removed
   --> tests/source/expr.rs:416:33
    |
    |
416 |       let requires = requires.set(box requires0
    |  _________________________________^
417 | |                                 .concat(&requires1)
418 | |                                 .concat(&requires2)
419 | |                                 .distinct_total());
    |
help: use `Box::new()` instead
    |
    |
416 ~     let requires = requires.set(Box::new(requires0
417 +                                 .concat(&requires1)
418 +                                 .concat(&requires2)
419 ~                                 .distinct_total()));

error: `box_syntax` has been removed
 --> tests/target/expr.rs:6:27
  |
  |
6 |     let boxed: Box<i32> = box 5;
  |
help: use `Box::new()` instead
  |
  |
6 |     let boxed: Box<i32> = Box::new(5);

error: `box_syntax` has been removed
   --> tests/target/expr.rs:185:13
    |
    |
185 |             box DocumentProgressHandler::new(addr, DocumentProgressTask::DOMContentLoaded);
    |
help: use `Box::new()` instead
    |
    |
185 |             Box::new(DocumentProgressHandler::new(addr, DocumentProgressTask::DOMContentLoaded));

error: `box_syntax` has been removed
   --> tests/target/expr.rs:458:9
    |
    |
458 | /         box requires0
459 | |             .concat(&requires1)
460 | |             .concat(&requires2)
461 | |             .distinct_total(),
    |
help: use `Box::new()` instead
    |
458 ~         Box::new(requires0
458 ~         Box::new(requires0
459 +             .concat(&requires1)
460 +             .concat(&requires2)
461 ~             .distinct_total()),

test test::self_tests ... ok
test test::system_tests ... FAILED
error: `box_syntax` has been removed
error: `box_syntax` has been removed
   --> tests/target/configs/combine_control_expr/false.rs:112:9
    |
112 |       foo(box Bar {
113 | |         aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
114 | |         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,
115 | |     });
    | |_____^
    | |_____^
    |
help: use `Box::new()` instead
    |
112 ~     foo(Box::new(Bar {
114 +         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,
115 ~     }));
    |


error: `box_syntax` has been removed
   --> tests/target/configs/combine_control_expr/true.rs:100:9
    |
100 |       foo(box Bar {
101 | |         aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
102 | |         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,
103 | |     });
    | |_____^
    | |_____^
    |
help: use `Box::new()` instead
    |
100 ~     foo(Box::new(Bar {
102 +         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,
103 ~     }));
    |


test test::idempotence_tests ... FAILED

failures:

---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
Warning: the `fn_args_layout` option is deprecated. Use `fn_params_layout`. instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10
---- test::idempotence_tests stdout ----
---- test::idempotence_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
Warning: the `fn_args_layout` option is deprecated. Use `fn_params_layout`. instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `3`,
  left: `3`,
 right: `0`: 3 idempotent tests failed', src/tools/rustfmt/src/test/mod.rs:369:9
thread 'test::idempotence_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10

failures:
    test::idempotence_tests
    test::system_tests
