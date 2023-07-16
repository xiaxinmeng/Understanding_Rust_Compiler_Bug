
~/Downloads $ rustc test.rs
test.rs:2:1: 4:3 warning: static item is never used: `ASCII_CLASSES`, #[warn(dead_code)] on by default
test.rs:2 static ASCII_CLASSES: NamedClasses = &[
test.rs:3      ("alnum", &ALNUM),
test.rs:4 ];
test.rs:5:1: 5:79 warning: static item is never used: `ALNUM`, #[warn(dead_code)] on by default
test.rs:5 static ALNUM: &'static [(char, char)] = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fish: Job 1, 'rustc test.rs ' terminated by signal SIGSEGV (Address boundary error)
