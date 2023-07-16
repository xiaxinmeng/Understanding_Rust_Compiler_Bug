
/home/nmatsakis/tmp/issue-31997.rs:10:21: 10:24 error: unresolved name `bar` [E0425]
/home/nmatsakis/tmp/issue-31997.rs:10     try!(closure(|| bar(0 as *mut _))) // bar intentionally undefined
                                                          ^~~
/home/nmatsakis/tmp/issue-31997.rs:10:5: 10:39 note: in this expansion of try! (defined in <std macros>)
/home/nmatsakis/tmp/issue-31997.rs:10:21: 10:24 help: run `rustc --explain E0425` to see a detailed explanation
<std macros>:5:8: 6:42 error: mismatched types:
 expected `()`,
    found `core::result::Result<_, _>`
(expected (),
    found enum `core::result::Result`) [E0308]
<std macros>:5 return $ crate:: result:: Result:: Err (
<std macros>:6 $ crate:: convert:: From:: from ( err ) ) } } )
/home/nmatsakis/tmp/issue-31997.rs:10:5: 10:39 note: in this expansion of try! (defined in <std macros>)
<std macros>:5:8: 6:42 help: run `rustc --explain E0308` to see a detailed explanation
/home/nmatsakis/tmp/issue-31997.rs:10:10: 10:17 error: the trait `TheTrait` is not implemented for the type `()` [E0277]
/home/nmatsakis/tmp/issue-31997.rs:10     try!(closure(|| bar(0 as *mut _))) // bar intentionally undefined
                                               ^~~~~~~
/home/nmatsakis/tmp/issue-31997.rs:10:5: 10:39 note: in this expansion of try! (defined in <std macros>)
/home/nmatsakis/tmp/issue-31997.rs:10:10: 10:17 help: run `rustc --explain E0277` to see a detailed explanation
/home/nmatsakis/tmp/issue-31997.rs:10:10: 10:17 note: required by `closure`
/home/nmatsakis/tmp/issue-31997.rs:10:25: 10:36 error: casting `usize` as `*mut _` is invalid
/home/nmatsakis/tmp/issue-31997.rs:10     try!(closure(|| bar(0 as *mut _))) // bar intentionally undefined
                                                              ^~~~~~~~~~~
/home/nmatsakis/tmp/issue-31997.rs:10:5: 10:39 note: in this expansion of try! (defined in <std macros>)
error: aborting due to 3 previous errors
