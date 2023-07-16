 rust
#![feature(macro_rules)]

macro_rules! print_doccomment(
  (
    $doc:tt
    fn $name:ident()
  ) => (
    macro_rules! dummy( () => (
      $doc
      fn $name() {
        println!("The doc for this function is `{}`", stringify!($doc));
      }
    ))
    dummy!()
  )
);

print_doccomment!(
  /// This is a doc comment
  fn function_to_doc()
);

fn main() {
  function_to_doc();
}
