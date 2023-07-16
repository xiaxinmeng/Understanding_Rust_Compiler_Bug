 rust
enum MakePanic {
  Panic(usize),
  Calm,
  Anxious,
}

#[test]
fn test_compile_panic() {
  let mkpnc = MakePanic::Panic(3);
  let MakePanic::Panic(val) = mkpnc;
  println!("Use val {}", val);
}
