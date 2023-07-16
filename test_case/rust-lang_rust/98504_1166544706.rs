
error: Undefined Behavior: deallocating while item is protected: [SharedReadOnly for <45060> (call 13916)]
  --> atomic.rs:11:1
   |
11 | / std::thread::scope(|s| {
12 | |     for t in &some_bools[..5] {
13 | |         s.spawn(move || assert_eq!(t.load(Ordering::Relaxed), true));
14 | |     }
...  |
18 | |     }
19 | | });
   | |__^ deallocating while item is protected: [SharedReadOnly for <45060> (call 13916)]
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
           
   = note: inside `main` at atomic.rs:11:1

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
