
let drop_flag = DropFlag::default();
let drop_flag_copy = drop_flag.clone();
let _ = drop_flag_copy;
drop_flag.assert_dropped();
