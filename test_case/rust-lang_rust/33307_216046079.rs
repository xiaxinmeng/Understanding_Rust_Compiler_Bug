
closure-example.rs:6:13: 6:26 error: the trait bound `std::sync::mpsc::Receiver<()>: std::marker::Sync` is not satisfied [E0277]
closure-example.rs:6     let t = thread::spawn(|| {
                                 ^~~~~~~~~~~~~
closure-example.rs:6:13: 6:26 help: run `rustc --explain E0277` to see a detailed explanation
closure-example.rs:6:13: 6:26 note: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
closure-example.rs:6:13: 6:26 note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
closure-example.rs:6:13: 6:26 note: required because it appears within the type `[closure@closure-example.rs:6:27: 8:6 recv:&std::sync::mpsc::Receiver<()>]`
closure-example.rs:6:13: 6:26 note: required by `std::thread::spawn`
error: aborting due to previous error
