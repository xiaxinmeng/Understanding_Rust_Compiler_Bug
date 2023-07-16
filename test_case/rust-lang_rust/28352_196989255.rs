
<anon>:11:1: 14:2 error: duplicate lang item found: `panic`. [E0152]
<anon>:11 extern fn panic(expr_file_line: &(&'static str, &'static str, u32)) -> ! {
<anon>:12     let (expr, file, line) = *expr_file_line;
<anon>:13     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]), &(file, line))
<anon>:14 }
<anon>:11:1: 14:2 help: see the detailed explanation for E0152
note: first defined in crate `core`.
error: aborting due to previous error
playpen: application terminated with error code 101
