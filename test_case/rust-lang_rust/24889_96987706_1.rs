
test2.rs:1:1: 3:2 error: not all control paths return a value [E0269]
test2.rs:1 fn thing() -> u32 {
test2.rs:2     7u32;
test2.rs:3 }
test2.rs:2:9: 2:10 help: consider removing this semicolon:
test2.rs:2     7u32;
                   ^
error: aborting due to previous error
