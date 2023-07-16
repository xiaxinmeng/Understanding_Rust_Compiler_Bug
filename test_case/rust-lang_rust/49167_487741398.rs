
error[E0596]: cannot borrow `*new` as mutable, as it is behind a `&` reference
 --> src/main.rs:6:5
  |
3 |     let mut new = &format!("This prints: {}", greeting).to_string();
  |                   ------------------------------------------------- help: consider changing this to be a mutable reference: `&mut format!("This prints: {}", greeting).to_string()`
...
6 |     new.push_str(&format!("oh no {}", ":("));
  |     ^^^ `new` is a `&` reference, so the data it refers to cannot be borrowed as mutable
