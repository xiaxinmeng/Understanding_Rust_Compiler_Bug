
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25:17: 25:31 error: borrowed value does not live long enough
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25     let fubar = regex!("abc"); //~ ERROR unused variable: `fubar`
                                                                                                         ^~~~~~~~~~~~~~
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:13:1: 26:1 note: in expansion of regex!
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25:17: 25:31 note: expansion site
note: reference must be valid for the static lifetime...
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25:5: 25:30 note: ...but borrowed value is only valid for the statement at 25:4
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25     let fubar = regex!("abc"); //~ ERROR unused variable: `fubar`
                                                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25:5: 25:30 help: consider using a `let` binding to increase its lifetime
/home/alex/code/rust2/src/test/compile-fail-fulldeps/syntax-extension-regex-unused.rs:25     let fubar = regex!("abc"); //~ ERROR unused variable: `fubar`
                                                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
