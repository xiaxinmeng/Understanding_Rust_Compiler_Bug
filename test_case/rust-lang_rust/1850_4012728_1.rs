
  #[external(c_src = "foo_glue.c", link_name = "foo_bar")]
  fn c_foo_bar(baz: *u8, quux: uint)
  