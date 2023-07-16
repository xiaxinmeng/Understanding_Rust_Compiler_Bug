
warning: a `const` item should never be interior mutable
  --> src/main.rs:18:1
   |
18 |   const RE: Lazy<Regex> = Lazy::new(|| {
   |   ^----
   |   |
   |  _make this a static item (maybe with lazy_static)
   | |
19 | |     println!("initializing regex");
20 | |     Regex::new(r"(?P<p>\d+)(?:_)(?P<i>\d+)(?:_)(?P<j>\d+)(?:\.png)").unwrap()
21 | | });
   | |___^
   |
   = note: `#[warn(clippy::declare_interior_mutable_const)]` on by default
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#declare_interior_mutable_const

warning: a `const` item with interior mutability should not be borrowed
  --> src/main.rs:46:16
   |
46 |     let caps = RE.captures(&text).unwrap();
   |                ^^
   |
   = note: `#[warn(clippy::borrow_interior_mutable_const)]` on by default
   = help: assign this const to a local or static variable, and use the variable here
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#borrow_interior_mutable_const
