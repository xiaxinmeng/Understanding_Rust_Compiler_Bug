
error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
 --> src/main.rs:6:5
  |
6 |     thread::spawn(|| tx.send(()).unwrap());
  |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
  |
  = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
  = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
  = note: required because it appears within the type `[closure@src/main.rs:6:19: 6:42 tx:&std::sync::mpsc::Sender<()>]`
  = note: required by `std::thread::spawn`
