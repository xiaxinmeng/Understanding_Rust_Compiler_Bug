
error[E0277]: `&&mut Vec<i32>` is not an iterator
 --> src/main.rs:2:14
  |
2 |     for x in &v {}
  |              ^^ `&&mut Vec<i32>` is not an iterator
  |
  = help: the trait `Iterator` is not implemented for `&&mut Vec<i32>`
  = help: the following other types implement trait `IntoIterator`:
            &'a Vec<T, A>
            &'a mut Vec<T, A>
            Vec<T, A>
  = note: required for `&&mut Vec<i32>` to implement `IntoIterator`
