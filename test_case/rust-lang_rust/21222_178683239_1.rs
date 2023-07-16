
hello.rs:2:8: 2:11 error: unresolved name `foo` [E0425]
hello.rs:2     if foo {}.bar()
                  ^~~
hello.rs:2:8: 2:11 help: run `rustc --explain E0425` to see a detailed explanation
error: aborting due to previous error

shell returned 101

Press ENTER or type command to continue
hello.rs:3:15: 3:16 error: unexpected token: `+`
hello.rs:3     if foo {} + bar
                         ^~
