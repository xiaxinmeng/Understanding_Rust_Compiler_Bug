
test.rs:11:21: 11:42 error: failed to find an implementation of trait core::ops::Fn<int,int> for fn(int) -> int
test.rs:11     println!("{}", (&foo as &Fn<int, int>).call(1));
                               ^~~~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:23: 2:77 note: expansion site
<std macros>:1:1: 3:2 note: in expansion of println!
test.rs:11:5: 11:53 note: expansion site
