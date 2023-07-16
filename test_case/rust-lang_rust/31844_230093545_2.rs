
error: mismatched types [--explain E0308]
  --> <anon>:15:9
   |>
15 |>         self.collect::<Vec<_>>().into_iter()
   |>         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found struct `std::vec::IntoIter`
note: expected type `<T as ClonableIterator>::ClonableIter`
note:    found type `std::vec::IntoIter<<T as std::iter::Iterator>::Item>`
