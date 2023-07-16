text
error[E0277]: the trait bound `std::sync::mpsc::Sender<()>: std::marker::Sync` is not satisfied
 --> <anon>:6:5
  |
6 |     thread::spawn(|| tx.send(()).unwrap());
  |     ^^^^^^^^^^^^^ the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
  |
  = note: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
