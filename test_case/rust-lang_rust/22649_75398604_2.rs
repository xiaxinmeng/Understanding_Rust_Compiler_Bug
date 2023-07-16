
<std macros>:5:10: 5:35 error: the trait `core::cmp::PartialEq<collections::vec::Vec<u8>>` is not implemented for the type `[u8]` [E0277]
<std macros>:5 if ! ( ( * left_val == * right_val ) && ( * right_val == * left_val ) ) {
                        ^~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
<anon>:3:3: 3:24 note: expansion site
<std macros>:5:43: 5:68 error: the trait `core::cmp::PartialEq<[u8]>` is not implemented for the type `collections::vec::Vec<u8>` [E0277]
<std macros>:5 if ! ( ( * left_val == * right_val ) && ( * right_val == * left_val ) ) {
                                                         ^~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
<anon>:3:3: 3:24 note: expansion site
error: aborting due to 2 previous errors
playpen: application terminated with error code 101
