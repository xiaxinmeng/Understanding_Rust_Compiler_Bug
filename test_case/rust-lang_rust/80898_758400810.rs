
2021-01-11T10:16:38.9893327Z ------------------------------------------
2021-01-11T10:16:38.9893804Z stderr:
2021-01-11T10:16:38.9894236Z ------------------------------------------
2021-01-11T10:16:38.9894917Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2021-01-11T10:16:38.9895503Z   left: `1`,
2021-01-11T10:16:38.9895913Z  right: `2`', debug.rs:18:5
2021-01-11T10:16:38.9896595Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2021-01-11T10:16:38.9897518Z thread '<unnamed>' panicked at 'assertion failed: { hit = true; false }', debug.rs:25:5
2021-01-11T10:16:38.9898807Z thread '<unnamed>' panicked at 'attempt to add with overflow', debug.rs:30:34
2021-01-11T10:16:38.9900103Z error: linking with `link.exe` failed: exit code: 1201

...

2021-01-11T10:16:38.9954834Z error: aborting due to previous error
2021-01-11T10:16:38.9955731Z make[1]: *** [Makefile:12: all] Error 1
2021-01-11T10:16:38.9956579Z ------------------------------------------
2021-01-11T10:16:38.9957646Z failures:
2021-01-11T10:16:38.9958447Z     [run-make] run-make-fulldeps\debug-assertions
2021-01-11T10:16:38.9959711Z test result: FAILED. 170 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out; finished in 46.67s
