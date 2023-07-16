
test-span-lint.rs:15:10: 15:12 warning: starts here
test-span-lint.rs:15     foo!(1u);
                              ^~
test-span-lint.rs:5:1: 7:2 note: in expansion of bar!
test-span-lint.rs:10:21: 10:29 note: expansion site
test-span-lint.rs:9:1: 12:2 note: in expansion of foo!
test-span-lint.rs:15:5: 15:14 note: expansion site
test-span-lint.rs:10:21: 10:29 note: continues
test-span-lint.rs:10     ($e: expr) => { bar!($e) };
                                         ^~~~~~~~
test-span-lint.rs:9:1: 12:2 note: in expansion of foo!
test-span-lint.rs:15:5: 15:14 note: expansion site
test-span-lint.rs:15:5: 15:14 note: continues
test-span-lint.rs:15     foo!(1u);
                         ^~~~~~~~~
test-span-lint.rs:11:13: 11:15 warning: starts here
test-span-lint.rs:11     () => { 1u }
                                 ^~
test-span-lint.rs:9:1: 12:2 note: in expansion of foo!
test-span-lint.rs:16:5: 16:12 note: expansion site
test-span-lint.rs:16:5: 16:12 note: continues
test-span-lint.rs:16     foo!();
                         ^~~~~~~
