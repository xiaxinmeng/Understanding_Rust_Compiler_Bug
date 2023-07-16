 rust
struct LowerChars<'a> {
  chars: std::str::Chars<'a>
}

fn lower<'a>(s: &'a str) -> LowerChars<'a> {
  LowerChars { chars: s.chars() }
}

impl<'a> Iterator<char> for LowerChars<'a> {
  fn next(&mut self) -> Option<char> {
    self.chars.next().map(|c| c.to_lowercase())
  }
}

#[test]
fn test_to_uppercase(){
  let sl = "foobär";
  let su = "FOOBÄR";

  let mut z = lower(sl).zip(lower(su));
  assert!(z.all(|(x, y)| x == y ));

  let greek = "στιγμας".chars().map(|c| c.to_uppercase()).collect::<~str>();
  // fail!(greek);
  assert_eq!(greek.as_slice(), "ΣΤΙΓΜΑΣ");
}
