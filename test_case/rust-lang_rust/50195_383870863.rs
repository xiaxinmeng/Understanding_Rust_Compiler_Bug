
error[E0658]: non-reference pattern used to match a reference (see issue #42640)
 --> src/main.rs:5:39
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |                                       ^^^^^^ help: consider using a reference: `&(_, v)`
