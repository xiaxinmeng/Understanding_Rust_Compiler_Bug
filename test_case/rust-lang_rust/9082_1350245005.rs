
error[E0277]: a value of type `Vec<X>` cannot be built from an iterator over elements of type `&X`
    --> f54.rs:9:7
     |
9    |     i.collect()
     |       ^^^^^^^ value of type `Vec<X>` cannot be built from `std::iter::Iterator<Item=&X>`
     |
     = help: the trait `FromIterator<&X>` is not implemented for `Vec<X>`
     = help: the trait `FromIterator<T>` is implemented for `Vec<T>`
note: the method call chain might not have had the expected associated types
    --> f54.rs:7:15
     |
7    |     let i = s.iter();
     |             - ^^^^^^ `Iterator::Item` is `&X` here
     |             |
     |             this expression has type `&HashSet<X>`
8    |     let mut i = i.map(|x| x.clone());
     |                   ------------------ `Iterator::Item` remains `&X` here
note: required by a bound in `collect`
    --> /Users/ekuber/workspace/rust/library/core/src/iter/traits/iterator.rs:1832:19
     |
1832 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::collect`
