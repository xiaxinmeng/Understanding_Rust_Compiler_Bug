
error: cannot specialize on trait `core::iter::TrustedLen`
    --> src/liballoc/vec.rs:2049:1
     |
2049 | / impl<T, I> SpecFromNested<T, I> for Vec<T>
2050 | | where
2051 | |     I: TrustedLen<Item = T>,

error: cannot specialize on trait `core::iter::InPlaceIterable`
error: cannot specialize on trait `core::iter::SourceIter`
error: cannot specialize on trait `vec::AsIntoIter`
    --> src/liballoc/vec.rs:2147:1
     |
2147 | / impl<T, I> SpecFrom<T, I> for Vec<T>
2148 | | where
2149 | |     I: Iterator<Item = T> + InPlaceIterable + SourceIter<Source: AsIntoIter>,


error: cannot specialize on trait `core::marker::Copy`
    --> src/liballoc/vec.rs:2219:1
     |
2219 | / impl<'a, T: 'a> SpecFrom<&'a T, slice::Iter<'a, T>> for Vec<T>
2220 | | where
2221 | |     T: Copy,


error: cannot specialize on trait `core::iter::TrustedLen`
    --> src/liballoc/vec.rs:2247:1
     |
2247 | / impl<T, I> SpecExtend<T, I> for Vec<T>
2248 | | where
2249 | |     I: TrustedLen<Item = T>,


error: cannot specialize on trait `core::marker::Copy`
    --> src/liballoc/vec.rs:2299:1
     |
2299 | / impl<'a, T: 'a> SpecExtend<&'a T, slice::Iter<'a, T>> for Vec<T>
2300 | | where
2301 | |     T: Copy,
