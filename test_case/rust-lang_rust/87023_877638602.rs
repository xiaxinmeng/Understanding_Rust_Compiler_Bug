plain
    Checking hashbrown v0.11.0
    Checking object v0.22.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.14.0
error[E0277]: `&map::Iter<'_, K, V>` is not an iterator
     |
     |
1175 |         f.debug_list().entries(self.clone()).finish()
     |                                ^^^^^^^^^^^^ `&map::Iter<'_, K, V>` is not an iterator
     |
     = help: the trait `core::iter::Iterator` is not implemented for `&map::Iter<'_, K, V>`
     = note: `core::iter::Iterator` is implemented for `&mut collections::hash::map::Iter<'_, K, V>`, but not for `&collections::hash::map::Iter<'_, K, V>`
     = note: required because of the requirements on the impl of `IntoIterator` for `&map::Iter<'_, K, V>`

error[E0599]: the method `clone` exists for struct `map::Iter<'_, K, V>`, but its trait bounds were not satisfied
     |
     |
1168 | pub struct Iter<'a, K: 'a, V: 'a> {
     | |
     | |
     | method `clone` not found for this
     | doesn't satisfy `map::Iter<'_, K, V>: Clone`
...
1263 |         Keys { inner: self.inner.clone() }
     |                                  ^^^^^ method cannot be called on `map::Iter<'_, K, V>` due to unsatisfied trait bounds
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `K: Clone`
             which is required by `map::Iter<'_, K, V>: Clone`
             `V: Clone`
             which is required by `map::Iter<'_, K, V>: Clone`
help: consider restricting the type parameters to satisfy the trait bounds
     |
1168 | pub struct Iter<'a, K: 'a, V: 'a> where K: Clone, V: Clone {


error[E0599]: the method `clone` exists for struct `map::Iter<'_, K, V>`, but its trait bounds were not satisfied
     |
     |
1168 | pub struct Iter<'a, K: 'a, V: 'a> {
     | |
     | |
     | method `clone` not found for this
     | doesn't satisfy `map::Iter<'_, K, V>: Clone`
...
1300 |         Values { inner: self.inner.clone() }
     |                                    ^^^^^ method cannot be called on `map::Iter<'_, K, V>` due to unsatisfied trait bounds
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `K: Clone`
             which is required by `map::Iter<'_, K, V>: Clone`
             `V: Clone`
             which is required by `map::Iter<'_, K, V>: Clone`
help: consider restricting the type parameters to satisfy the trait bounds
     |
1168 | pub struct Iter<'a, K: 'a, V: 'a> where K: Clone, V: Clone {

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
