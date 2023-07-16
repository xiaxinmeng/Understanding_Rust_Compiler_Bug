rust
my_cell.get_or_try_init(|| fn_returning_option().ok_or("this will be a Err(&str)"))
