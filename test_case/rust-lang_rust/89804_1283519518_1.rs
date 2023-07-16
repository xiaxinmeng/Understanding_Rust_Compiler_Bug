
fn main() { 
  let pos: Vec<String> = vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()];
  let pos: &[&str] = pos.iter().map(AsRef::as_ref).collect::<Vec<_>>().as_slice();
  println!("{:?}", pos);
}
