
error[E0597]: `s` does not live long enough
 --> src/lib.rs:7:20
  |
7 |     let as_bytes = s.as_bytes();
  |                    ^^^^^^^^^^^^ borrowed value does not live long enough
8 |     path_content_map.insert("foo", (as_bytes.len() as u64, Box::new(as_bytes)));
9 | }
  | -
  | |
  | `s` dropped here while still borrowed
  | borrow might be used here, when `path_content_map` is dropped and runs the `Drop` code for type `BTreeMap`
  |
  = note: values in a scope are dropped in the opposite order they are defined

For more information about this error, try `rustc --explain E0597`.
error: could not compile `repor_new_rust_failure` due to previous error
