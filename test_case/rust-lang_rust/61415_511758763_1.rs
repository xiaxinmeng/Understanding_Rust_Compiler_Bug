rust
#![feature(const_generics)]
const fn len<T, const N: usize>(_: [T; N]) -> usize { N }
const FOO: usize = len([0]);
