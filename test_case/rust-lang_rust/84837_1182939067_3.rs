
error[E0277]: `Vec<{integer}>` is not an iterator
 --> src/main.rs:7:21
  |
7 |         for item in items {
  |                     ^^^^^ `Vec<{integer}>` is not an iterator
  |
  = help: the trait `Iterator` is not implemented for `Vec<{integer}>`
  = help: the following other types implement trait `IntoIterator`:
            &'a Vec<T, A>
            &'a mut Vec<T, A>
            Vec<T, A>
  = note: required because of the requirements on the impl of `Iterator` for `&mut Vec<{integer}>`
  = note: 1 redundant requirement hidden
  = note: required because of the requirements on the impl of `~const Iterator` for `&mut &mut Vec<{integer}>`
  = note: required because of the requirements on the impl of `IntoIterator` for `&mut &mut Vec<{integer}>`
