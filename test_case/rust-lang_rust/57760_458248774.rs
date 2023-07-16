rust
let fmt_str = CString::new("%d%d\n").unwrap();
printf(fmt_str.as_ptr(), 0, 42);
