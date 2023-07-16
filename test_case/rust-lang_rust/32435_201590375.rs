
/home/nmatsakis/tmp/error-spew.rs:843:9: 843:10 error: incorrect close delimiter: `}`
/home/nmatsakis/tmp/error-spew.rs:843         } else {
                                              ^
/home/nmatsakis/tmp/error-spew.rs:841:21: 841:22 note: unclosed delimiter
/home/nmatsakis/tmp/error-spew.rs:841             callback(path.as_ref(); // MISSING )
                                                          ^
/home/nmatsakis/tmp/error-spew.rs:841:35: 841:36 error: expected one of `,`, `.`, `?`, or an operator, found `;`
/home/nmatsakis/tmp/error-spew.rs:841             callback(path.as_ref(); // MISSING )
                                                                        ^
/home/nmatsakis/tmp/error-spew.rs:842:13: 842:15 error: expected one of `.`, `;`, `?`, `}`, or an operator, found `fs`
/home/nmatsakis/tmp/error-spew.rs:842             fs::create_dir_all(path.as_ref()).map(|()| true)
                                                  ^~
/home/nmatsakis/tmp/error-spew.rs:841:13: 841:36 error: mismatched types:
 expected `core::result::Result<bool, std::io::error::Error>`,
    found `()`
(expected enum `core::result::Result`,
    found ()) [E0308]
/home/nmatsakis/tmp/error-spew.rs:841             callback(path.as_ref(); // MISSING )
                                                  ^~~~~~~~~~~~~~~~~~~~~~~
/home/nmatsakis/tmp/error-spew.rs:841:13: 841:36 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
