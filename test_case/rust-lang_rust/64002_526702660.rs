log
error[E0597]: `line` does not live long enough
  --> src/main.rs:24:30
   |
24 |         let spt: Vec<&str> = line.split(char::is_whitespace).collect();
   |                              ^^^^ borrowed value does not live long enough
...
27 |             "#mem" => { rule.mem.push((spt[1], spt[2])) }
   |                                       -------- borrow later used here
...
30 |     }
   |     - `line` dropped here while still borrowed
