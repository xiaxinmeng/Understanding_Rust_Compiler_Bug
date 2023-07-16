
error[E0659]: `assert_matches` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
   --> ../../src/lib/test_executor/rust/src/diagnostics.rs:363:13
    |
363 |             assert_matches!(
    |             ^^^^^^^^^^^^^^ ambiguous name
    |
    = note: `assert_matches` could refer to a macro from prelude
note: `assert_matches` could also refer to the macro imported here
   --> ../../src/lib/test_executor/rust/src/diagnostics.rs:322:13
    |
322 |             super::*,
    |             ^^^^^^^^
    = help: consider adding an explicit import of `assert_matches` to disambiguate
    = help: or use `self::assert_matches` to refer to this macro unambiguously
