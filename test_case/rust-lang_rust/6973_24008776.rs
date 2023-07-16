 rust
fn ga(bu: int, zo: Option<int>, meu: Option<int>) {
  let zo = zo.get_or_default(42);
  let meu = meu.get_or_default(99);
  ...
}
