
<anon>:5:20: 5:32 error: the type of this value must be known in this context
<anon>:5   let new_el = |v| v.insert(30);  // error: the type of this value must be known in this context
                            ^~~~~~~~~~~~
<anon>:5:16: 5:32 error: can't infer the "kind" of the closure, explicitly annotate it. e.g. `|&:| {}`
<anon>:5   let new_el = |v| v.insert(30);  // error: the type of this value must be known in this context
                        ^~~~~~~~~~~~~~~~
<anon>:11:18: 11:20 error: the trait `core::fmt::String` is not implemented for the type `std::collections::hash::map::HashMap<(u32, u32), u32>`
<anon>:11   println!("{}", xs);
                           ^~
note: in expansion of format_args!
<std macros>:2:42: 2:75 note: expansion site
<std macros>:1:1: 2:77 note: in expansion of println!
<anon>:11:3: 11:22 note: expansion site
error: aborting due to 3 previous errors
playpen: application terminated with error code 101
Program ended.
