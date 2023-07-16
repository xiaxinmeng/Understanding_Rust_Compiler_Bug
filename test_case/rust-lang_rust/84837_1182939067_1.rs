
error[E0277]: `[usize; 5]` is not an iterator
 --> src/main.rs:7:21
  |
7 |         for item in items {
  |                     ^^^^^ `[usize; 5]` is not an iterator; try calling `.into_iter()` or `.iter()`
  |
  = help: the trait `Iterator` is not implemented for `[usize; 5]`
  = help: the following other types implement trait `IntoIterator`:
            &'a [T; N]
            &'a [T]
            &'a mut [T; N]
            &'a mut [T]
            [T; N]
  = note: required because of the requirements on the impl of `Iterator` for `&mut [usize; 5]`
  = note: 1 redundant requirement hidden
  = note: required because of the requirements on the impl of `~const Iterator` for `&mut &mut [usize; 5]`
  = note: required because of the requirements on the impl of `IntoIterator` for `&mut &mut [usize; 5]`
