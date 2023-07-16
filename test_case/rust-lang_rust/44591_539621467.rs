
error[E0277]: the trait bound `<I as std::iter::Iterator>::Item: Collectable<std::vec::Vec<TypeA>>` is not satisfied
   --> src/main.rs:155:5
    |
155 | /     fn make_from_iter<I>(&self, iter: I)
156 | |     -> <<I::Item as Collectable<Vec<TypeA>>>::Collected as Mapper<(Resp, )>>::Res
157 | |         where I: Iterator,
158 | |               I::Item: Collectable<Vec<TypeA>>,
...   |
161 | |         I::Item::collect(iter).and_then(|_items| (Resp,))
162 | |     }
    | |_____^ the trait `Collectable<std::vec::Vec<TypeA>>` is not implemented for `<I as std::iter::Iterator>::Item`
    |
    = help: consider adding a `where <I as std::iter::Iterator>::Item: Collectable<std::vec::Vec<TypeA>>` bound

error[E0276]: impl has stricter requirements than trait
   --> src/main.rs:155:5
    |
143 | /     fn make_from_iter<I>(&self, iter: I)
144 | |     -> <<I::Item as Collectable<Self::Collection>>::Collected as Mapper<(Resp, )>>::Res
145 | |         where I: Iterator,
146 | |               I::Item: Collectable<Self::Collection>,
147 | |               <I::Item as Collectable<Self::Collection>>::Collected: Mapper<(Resp,)>;
    | |_____________________________________________________________________________________- definition of `make_from_iter` from trait
...
155 | /     fn make_from_iter<I>(&self, iter: I)
156 | |     -> <<I::Item as Collectable<Vec<TypeA>>>::Collected as Mapper<(Resp, )>>::Res
157 | |         where I: Iterator,
158 | |               I::Item: Collectable<Vec<TypeA>>,
...   |
161 | |         I::Item::collect(iter).and_then(|_items| (Resp,))
162 | |     }
    | |_____^ impl has extra requirement `<I as std::iter::Iterator>::Item: Collectable<std::vec::Vec<TypeA>>`
