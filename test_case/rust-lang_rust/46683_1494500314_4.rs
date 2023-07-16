
error[E0277]: `dyn Table` is not an iterator
   --> src/bin/table_of_chairs/main.rs:11:26
    |
11  |     assert!(print_stdout(table).is_ok());
    |             ------------ ^^^^^ `dyn Table` is not an iterator
    |             |
    |             required by a bound introduced by this call
    |
    = help: the trait `Iterator` is not implemented for `dyn Table`
    = help: the trait `Table` is implemented for `TableStruct`
    = note: required for `Box<dyn Table>` to implement `Iterator`
    = note: required for `Box<dyn Table>` to implement `Table`
note: required by a bound in `print_stdout`
