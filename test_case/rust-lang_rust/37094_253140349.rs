
error[E0119]: conflicting implementations of trait `SpecExtend<_>` for type `vec::Vec<_>`:
    --> src/libcollections/vec.rs:1597:1
     |
1538 | impl<I, T> SpecExtend<I> for Vec<T> where I: IntoIterator<Item=T> {
     | - first implementation here
...
1597 | impl <'a, T: 'a + Copy, I: IntoIterator<Item=&'a T>> SpecExtend<I> for Vec<T> {
     | ^ conflicting implementation for `vec::Vec<_>`

error[E0119]: conflicting implementations of trait `SpecExtend<&[_]>` for type `vec::Vec<_>`:
    --> src/libcollections/vec.rs:1603:1
     |
1538 | impl<I, T> SpecExtend<I> for Vec<T> where I: IntoIterator<Item=T> {
     | - first implementation here
...
1603 | impl<'a, T: Copy> SpecExtend<&'a[T]> for Vec<T> {
     | ^ conflicting implementation for `vec::Vec<_>`

