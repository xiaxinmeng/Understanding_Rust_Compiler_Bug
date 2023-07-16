
error: lifetime may not live long enough
 --> file.rs:7:9
  |
6 |     fn foo(&self) -> Box<Fn()> {
  |            - let's call the lifetime of this reference `'1`
7 |         Box::new(move || println!("{}", self.name))
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`
