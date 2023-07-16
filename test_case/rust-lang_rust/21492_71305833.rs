
main.rs:2:36: 2:41 error: unresolved name `hello`
main.rs:2   println!("Hello in English: {}", hello());
                                             ^~~~~
note: in expansion of format_args!
<std macros>:2:42: 2:75 note: expansion site
<std macros>:1:1: 2:77 note: in expansion of println!
main.rs:2:3: 2:45 note: expansion site
main.rs:3:44: 3:51 error: unresolved name `goodbye`
main.rs:3   println!("Goodbye in Japanese(日本語): {}", goodbye());
                                                        ^~~~~~~
note: in expansion of format_args!
<std macros>:2:42: 2:75 note: expansion site
<std macros>:1:1: 2:77 note: in expansion of println!
main.rs:3:3: 3:55 note: expansion site
error: aborting due to 2 previous errors

