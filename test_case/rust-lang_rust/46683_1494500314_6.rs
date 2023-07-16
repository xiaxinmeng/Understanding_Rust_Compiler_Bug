
error[E0277]: the size for values of type `dyn Table` cannot be known at compilation time
   --> src/bin/table_of_chairs/main.rs:11:26
    |
11  |     assert!(print_stdout(*table).is_ok());
    |             ------------ ^^^^^^ doesn't have a size known at compile-time
    |             |
    |             required by a bound introduced by this call
    |
    = help: the trait `Sized` is not implemented for `dyn Table`
note: required by a bound in `print_stdout`
