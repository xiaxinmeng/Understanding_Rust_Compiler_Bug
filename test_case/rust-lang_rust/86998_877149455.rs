
error[E0080]: evaluation of constant value failed
   --> /home/mara/rust/library/core/src/panicking.rs:102:9
    |
102 |         panic_str(msg);
    |         ^^^^^^^^^^^^^^
    |         |
    |         the evaluated program panicked at 'hello', src/main.rs:4:5
    |         inside `core::panicking::panic_fmt` at /home/mara/rust/library/core/src/panicking.rs:102:9
    | 
   ::: src/main.rs:4:5
    |
4   |     panic!("hello")
    |     --------------- inside `hey` at /home/mara/rust/library/core/src/panic.rs:38:9
...
7   | const X: u32 = hey();
    |                ----- inside `X` at src/main.rs:7:16
