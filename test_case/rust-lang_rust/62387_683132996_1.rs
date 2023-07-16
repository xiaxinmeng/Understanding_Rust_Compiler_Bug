`
error[E0596]: cannot borrow `*container` as mutable, as it is behind a `&` reference
  --> src/main.rs:25:62
   |
25 |        let a = &self.contained().iter().flat_map(|container| container.things()).cloned().collect::<Vec<PathBuf>>();
   |                                                   ---------  ^^^^^^^^^ `container` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |                                                   |
   |                                                   help: consider changing this to be a mutable reference: `&mut Container`

error: aborting due to previous error; 1 warning emitted
