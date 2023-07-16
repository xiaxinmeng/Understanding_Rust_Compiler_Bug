
y.rs:2:21: 2:30 error: unresolved name
y.rs:2     ($arg:ident) => (foo::$arg);
                            ^~~~~~~~~
y.rs:1:0: 3:1 note: in expansion of test!
y.rs:6:4: 6:15 note: expansion site
y.rs:2:21: 2:30 error: use of undeclared module `foo`
y.rs:2     ($arg:ident) => (foo::$arg);
                            ^~~~~~~~~
y.rs:1:0: 3:1 note: in expansion of test!
y.rs:6:4: 6:15 note: expansion site
y.rs:2:21: 2:30 error: unresolved name: `foo::bar`.
y.rs:2     ($arg:ident) => (foo::$arg);
                            ^~~~~~~~~
y.rs:1:0: 3:1 note: in expansion of test!
y.rs:6:4: 6:15 note: expansion site
error: aborting due to 3 previous errors
