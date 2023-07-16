 shell
$ rustc main.rs
main.rs:2:5: 4:6 error: non-exhaustive patterns: `_` not covered [E0004]
main.rs:2     match 1 {
main.rs:3         1 => {}
main.rs:4     }
main.rs:2:5: 4:6 help: pass `--explain E0004` to see a detailed explanation
error: aborting due to previous error
