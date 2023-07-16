
error[E0277]: `&str` is not an iterator
   --> src/main.rs:2:14
    |
2   |     for x in "hello" {}
    |              ^^^^^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
    |
    = help: the trait `Iterator` is not implemented for `&str`
    = note: required because of the requirements on the impl of `IntoIterator` for `&str`
note: required by `into_iter`
