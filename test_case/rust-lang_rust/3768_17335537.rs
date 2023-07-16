 rust
let a = @mut A { a: None }; let mut v = Some(a); a.a <-> v;
  io::println(fmt!("%?",a));
