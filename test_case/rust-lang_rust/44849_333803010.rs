rust
let s: &'static str = "abc";
match &s {
  "abc" => true,
  _ => panic!(),
};
