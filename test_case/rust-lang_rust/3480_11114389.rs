
../src/test/run-pass/issue-3480.rs:13:29: 13:45 error: binary operation == cannot be applied to type `'a`
../src/test/run-pass/issue-3480.rs:13         vec::find(self, |e| {e.first() == key}).is_some()
                                                                   ^~~~~~~~~~~~~~~~
../src/test/run-pass/issue-3480.rs:13:28: 13:46 error: mismatched types: expected `bool` but found `'a` (expected bool but found type parameter)
../src/test/run-pass/issue-3480.rs:13         vec::find(self, |e| {e.first() == key}).is_some()
