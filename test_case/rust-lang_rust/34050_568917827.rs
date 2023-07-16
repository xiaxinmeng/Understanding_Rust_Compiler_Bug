text
error[E0597]: `v` does not live long enough
  --> src/main.rs:15:13
   |
14 |         let mut process = || {
   |                           -- value captured here
15 |             v.push("A".to_string());
   |             ^ borrowed value does not live long enough
...
20 |         do_some_work(&mut process);
   |                      ------------ cast requires that `v` is borrowed for `'static`
...
24 | }
   | - `v` dropped here while still borrowed

