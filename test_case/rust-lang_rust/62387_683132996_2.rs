`
error[E0308]: mismatched types
  --> src/main.rs:25:51
   |
25 |        let a = &self.contained().iter().flat_map(|&mut container| container.things()).cloned().collect::<Vec<PathBuf>>();
   |                                                   ^^^^^----------
   |                                                   |    |
   |                                                   |    expected due to this
   |                                                   types differ in mutability
   |                                                   help: did you mean `container`: `&&Container`
   |
   = note:      expected reference `&Container`
           found mutable reference `&mut _`

