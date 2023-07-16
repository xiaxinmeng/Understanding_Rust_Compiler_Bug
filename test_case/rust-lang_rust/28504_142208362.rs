
privacy-ufcs.rs:22:13: 22:29 error: method `baz` is inaccessible
privacy-ufcs.rs:22     <i32 as ::foo::Bar>::baz(); //~ERROR method `baz` is inaccessible
                               ^~~~~~~~~~~~~~~~
privacy-ufcs.rs:22:13: 22:29 note: trait `Bar` is private
privacy-ufcs.rs:22     <i32 as ::foo::Bar>::baz(); //~ERROR method `baz` is inaccessible
                               ^~~~~~~~~~~~~~~~
error: aborting due to previous error
