
$ RUST_LOG=::rt::backtrace rustc 6455.rs
6455.rs:9:10: 9:14 warning: variable does not need to be mutable [-W unused_mut]
6455.rs:9   let mut node  = Node{ number : 10, next : None };
                    ^~~~
rust: task 7f6068204720 ran out of stack
