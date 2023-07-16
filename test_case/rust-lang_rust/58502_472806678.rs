
% cargo clean && cargo +1.32.0 build
   Compiling bar v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue58502/bar)
   Compiling client v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue58502/client)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
% cargo clean && cargo +1.33.0 build
   Compiling bar v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue58502/bar)
   Compiling client v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue58502/client)
warning: function `AnotherNonSnakeCaseName` should have a snake case name
 --> src/main.rs:6:32
  |
6 | snakes_on_a_case!(global_allow AnotherNonSnakeCaseName);
  |                                ^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `another_non_snake_case_name`
  |
  = note: #[warn(non_snake_case)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
%
