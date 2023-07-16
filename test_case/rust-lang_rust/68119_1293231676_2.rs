
error[[E0507]](https://doc.rust-lang.org/stable/error-index.html#E0507): cannot move out of `cloned_value`, a captured variable in an `FnMut` closure
  --> src/lib.rs:15:20
   |
13 |       let cloned_value = value.clone();
   |           ------------ captured outer variable
14 |       arr.into_iter().map(move |_e| {
   |                           --------- captured by this `FnMut` closure
15 |           async move {
   |  ____________________^
16 | |             let value = cloned_value.clone();
   | |                         ------------
   | |                         |
   | |                         variable moved due to use in generator
   | |                         move occurs because `cloned_value` has type `Arc<Mutex<Value>>`, which does not implement the `Copy` trait
17 | |             println!("{:?}", value.lock().ok())
18 | |         }
   | |_________^ move out of `cloned_value` occurs here
   |
help: try cloning `cloned_value` before the `async move` block:
   |
13 |       let cloned_value = value.clone();
14 |       arr.into_iter().map(move |_e| {
15 |           let value = cloned_value.clone();
16 |           async move {
17 |               println!("{:?}, value.lock().ok())
18 |           }
