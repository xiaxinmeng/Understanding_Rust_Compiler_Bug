


<anon>:7:24: 7:28 warning: the `uint` type is deprecated; use `usize` or a fixed-sized integer
<anon>:7 static NUM_CATEGORIES: uint = Category::NumCategories as uint;
                                ^~~~
<anon>:7:24: 7:28 help: add #![feature(int_uint)] to the crate attributes to silence this warning
<anon>:7 static NUM_CATEGORIES: uint = Category::NumCategories as uint;
                                ^~~~
<anon>:7:58: 7:62 warning: the `uint` type is deprecated; use `usize` or a fixed-sized integer
<anon>:7 static NUM_CATEGORIES: uint = Category::NumCategories as uint;
                                                                  ^~~~
<anon>:7:58: 7:62 help: add #![feature(int_uint)] to the crate attributes to silence this warning
<anon>:7 static NUM_CATEGORIES: uint = Category::NumCategories as uint;
                                                                  ^~~~
<anon>:10:25: 10:51 error: expected constant expr for array length: non-constant path in constant expr [E0250]
<anon>:10     let the_categories: [Category; NUM_CATEGORIES] = [Category::A, Category::B, Category::C];
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~
playpen: application terminated with error code 101
Program ended.
