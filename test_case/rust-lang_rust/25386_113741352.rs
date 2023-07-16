
test.rs:19:9: 19:33 error: field `name` of struct `stuff::CObj` is private
test.rs:19         (*$var.c_object).$member.is_some()
                   ^~~~~~~~~~~~~~~~~~~~~~~~
test.rs:17:1: 21:2 note: in expansion of check_ptr_exist!
test.rs:25:20: 25:48 note: expansion site
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:25:5: 25:50 note: expansion site
test.rs:25:37: 19:24 error: field `c_object` of struct `stuff::Item` is private
(internal compiler error: unprintable span)
test.rs:17:1: 21:2 note: in expansion of check_ptr_exist!
test.rs:25:20: 25:48 note: expansion site
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:25:5: 25:50 note: expansion site
error: aborting due to 2 previous errors
