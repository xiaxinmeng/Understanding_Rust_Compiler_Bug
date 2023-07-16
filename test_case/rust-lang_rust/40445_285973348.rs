rust
error: cannot borrow immutable field `f.v` as mutable
 --> file.rs:7:13
  |
6 |    let f = Foo { v: Vec::new() };
  |        - consider changing this to `mut f`
7 |    f.v.push("cat".to_string());
  |    ^^^
