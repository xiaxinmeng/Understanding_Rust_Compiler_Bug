
[01:14:17] failures:
[01:14:17] 
[01:14:17] ---- pattern::double_ended_regression_test stdout ----
[01:14:17] 	thread 'pattern::double_ended_regression_test' panicked at 'assertion failed: `(left == right)`
[01:14:17]   left: `[InRange(0, 1), Done, Done, Done]`,
[01:14:17]  right: `[InRange(0, 1), InRange(10, 11), InRange(5, 6), Done]`: alternating double ended search', /checkout/src/libcore/../libcore/tests/pattern.rs:270:5
[01:14:17] 
[01:14:17] 
[01:14:17] failures:
[01:14:17]     pattern::double_ended_regression_test
[01:14:17] 
[01:14:17] test result: FAILED. 704 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
