
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): `Vec<u8>` is not an iterator
 --> src/main.rs:3:14
  |
3 |     for _ in &mut &mut v {}
  |              ^^^^^^^^^^^ `Vec<u8>` is not an iterator; try calling `.into_iter()` or `.iter()`
  |
  = help: the trait `Iterator` is not implemented for `Vec<u8>`
  = help: the following other types implement trait `IntoIterator`:
            &'a Vec<T, A>
            &'a mut Vec<T, A>
            Vec<T, A>
  = note: required for `&mut Vec<u8>` to implement `Iterator`
  = note: 1 redundant requirement hidden
  = note: required for `&mut &mut Vec<u8>` to implement `Iterator`
  = note: required for `&mut &mut Vec<u8>` to implement `IntoIterator`

error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): `[u8]` is not an iterator
 --> src/main.rs:6:14
  |
6 |     for _ in &mut v {}
  |              ^^^^^^ `[u8]` is not an iterator; try calling `.into_iter()` or `.iter()`
  |
  = help: the trait `Iterator` is not implemented for `[u8]`
  = help: the following other types implement trait `IntoIterator`:
            &'a [T; N]
            &'a [T]
            &'a mut [T; N]
            &'a mut [T]
            [T; N]
  = note: required for `&mut [u8]` to implement `Iterator`
  = note: 1 redundant requirement hidden
  = note: required for `&mut &mut [u8]` to implement `Iterator`
  = note: required for `&mut &mut [u8]` to implement `IntoIterator`
