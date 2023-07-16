
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): `<_ as FnOnce<(&'any GameState,)>>::Output` is not an iterator
  --> src/main.rs:43:13
   |
43 |     let _ = choose_randomly/*::<LeSigh, _>*/(&mut game, legal_actions::evaluate as _);
   |             ^^^^^^^^^^^^^^^ `<_ as FnOnce<(&'any GameState,)>>::Output` is not an iterator
   |
   = help: the trait `for<'any> Iterator` is not implemented for `<_ as FnOnce<(&'any GameState,)>>::Output`
note: required by a bound in `choose_randomly`
  --> src/main.rs:27:58
   |
21 | fn choose_randomly<G, Item>(
   |    --------------- required by a bound in this
...
27 |     for<'any> <G as FnOnce<(&'any GameState,)>>::Output: Iterator<Item = Item> + 'any,
   |
