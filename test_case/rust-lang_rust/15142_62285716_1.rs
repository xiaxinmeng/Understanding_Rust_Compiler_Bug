
a.rs:5:5: 7:6 help: consider using an explicit lifetime parameter as shown: fn next<'a>(&'a mut self) -> Option<&'a int>
a.rs:5     fn next<'a>(&'a mut self) -> Option<&'a int> {
a.rs:6         Iterator::next(self)
a.rs:7     }
a.rs:6:9: 6:29 error: mismatched types: expected `core::option::Option<&'a int>`, found `core::option::Option<&'a int>` (lifetime mismatch)
a.rs:6         Iterator::next(self)
               ^~~~~~~~~~~~~~~~~~~~
a.rs:6:9: 6:29 error: mismatched types: expected `core::option::Option<&'a int>`, found `core::option::Option<&'a int>` (lifetime mismatch)
a.rs:6         Iterator::next(self)
               ^~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
