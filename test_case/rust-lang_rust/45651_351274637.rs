
error[E0277]: the trait bound `{integer}: std::ops::Placer<_>` is not satisfied
 --> src/main.rs:4:8
  |
4 |     if x<-1 {
  |        ----
  |        |
  |        the trait `std::ops::Placer<_>` is not implemented for `{integer}`
  |        in this macro invocation
  |
  = help: the following implementations were found:
            <std::vec::PlaceBack<'a, T> as std::ops::Placer<T>>
            <std::collections::linked_list::FrontPlace<'a, T> as std::ops::Placer<T>>
            <std::collections::vec_deque::PlaceFront<'a, T> as std::ops::Placer<T>>
            <std::collections::vec_deque::PlaceBack<'a, T> as std::ops::Placer<T>>
          and 4 others
  = note: required by `std::ops::Placer::make_place`
