
error[E0221]: ambiguous associated type `Item` in bounds of `Self`
 --> <anon>:4:25
  |
2 |     type Item;
  |     ---------- ambiguous `Item` from `AnyFilter`
3 |     fn any_filter<F>(self, filters: &[F]) -> AnyFilterImpl<F, Self>
4 |         where F: FnMut(&Self::Item) -> bool,
  |                         ^^^^^^^^^^ ambiguous associated type `Item`
  |
note: associated type `Self` could derive from `std::iter::Iterator`
 --> <anon>:4:25
  |
4 |         where F: FnMut(&Self::Item) -> bool,
  |                         ^^^^^^^^^^
