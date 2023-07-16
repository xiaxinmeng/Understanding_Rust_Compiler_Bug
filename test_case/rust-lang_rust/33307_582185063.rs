
error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
   --> file4.rs:6:13
    |
6   |     let t = thread::spawn(|| {
    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
    |
   ::: /Users/ekuber/workspace/rust/src/libstd/thread/mod.rs:616:8
    |
616 |     F: Send + 'static,
    |        ---- required by this bound in `std::thread::spawn`
    |
    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
    = note: required because it appears within the type `[closure@file4.rs:6:27: 8:6 recv:&std::sync::mpsc::Receiver<()>]`
