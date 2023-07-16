
<anon>:6:7: 6:16 error: the trait `core::iter::FromIterator<&X>` is not implemented for the type `collections::vec::Vec<X>` [E0277]
<anon>:6     i.collect()
               ^~~~~~~~~
<anon>:6:7: 6:16 note: a collection of type `collections::vec::Vec<X>` cannot be built from an iterator over elements of type `&X`
<anon>:6     i.collect()
               ^~~~~~~~~
