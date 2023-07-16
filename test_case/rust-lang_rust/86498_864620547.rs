bash

raider@Ultra:~/rust/projects/guessing_game$ cargo clean
raider@Ultra:~/rust/projects/guessing_game$ cargo build
   Compiling libc v0.2.94
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (/home/raider/rust/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 11.89s

raider@Ultra:~/rust/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
4
You guessed: 4

