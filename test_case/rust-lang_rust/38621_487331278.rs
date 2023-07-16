
error[E0597]: `line` does not live long enough
  --> src/main.rs:34:24
   |
34 |         let mut line = line.trim();     
   |                        ^^^^ borrowed value does not live long enough
...
48 |             } else if let Some(label) = label_map.insert(label, instruction_address) {
   |                                         --------- borrow later used here
...
55 |     }
   |     - `line` dropped here while still borrowed
