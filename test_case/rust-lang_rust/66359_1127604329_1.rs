rust
let it = (vec![()], 42);
drop(it.0);
dbg!(type_name_of!(it));
