
error[E0308]: mismatched types
  --> f54.rs:12:16
   |
12 |     let h: H = h.clone();
   |            -   ^^^^^^^^^ expected struct `HashMap`, found reference
   |            |
   |            expected due to this
   |
   = note: expected struct `HashMap<String, Vec<(Foo, Bar)>>`
           found reference `&HashMap<String, Vec<(Foo, Bar)>>`
note: `HashMap<String, Vec<(Foo, Bar)>>` does not implement `Clone`, so `&HashMap<String, Vec<(Foo, Bar)>>` was cloned instead
  --> f54.rs:12:16
   |
12 |     let h: H = h.clone();
   |                ^
