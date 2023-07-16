 text
hello.rs:4:26: 4:33 error: chained comparison operators require parentheses
hello.rs:4         let mut foo = Vec<i32>::new(); // Remove <32i> to fix
                                    ^~~~~~~
hello.rs:4:26: 4:33 help: use `::<...>` instead of `<...>` if you meant to specify type arguments
hello.rs:4:23: 4:26 error: `Vec` is the name of a struct or struct variant, but this expression uses it like a function name [E0423]
hello.rs:4         let mut foo = Vec<i32>::new(); // Remove <32i> to fix
                                 ^~~
