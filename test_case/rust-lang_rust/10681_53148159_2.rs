
issue-10681.rs:5:9: 5:15 error: found `struct` in ident position
issue-10681.rs:5         struct Foo;
                         ^~~~~~
issue-10681.rs:5:16: 5:19 error: macro expansion ignores token `Foo` and any following
issue-10681.rs:5         struct Foo;
                                ^~~
error: aborting due to 2 previous errors
