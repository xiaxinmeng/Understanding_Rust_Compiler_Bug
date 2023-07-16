
/home/nmatsakis/tmp/issue-27592.rs:14:27: 14:31 error: borrowed value does not live long enough
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                                                ^~~~
note: in expansion of format_args!
/home/nmatsakis/tmp/issue-27592.rs:14:14: 14:47 note: expansion site
note: in expansion of closure expansion
/home/nmatsakis/tmp/issue-27592.rs:14:11: 14:47 note: expansion site
/home/nmatsakis/tmp/issue-27592.rs:14:5: 14:48 note: reference must be valid for the call at 14:4...
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/nmatsakis/tmp/issue-27592.rs:14:14: 14:47 note: ...but borrowed value is only valid for the block at 14:13
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/nmatsakis/tmp/issue-27592.rs:14:33: 14:46 error: borrowed value does not live long enough
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                                                      ^~~~~~~~~~~~~
note: in expansion of format_args!
/home/nmatsakis/tmp/issue-27592.rs:14:14: 14:47 note: expansion site
note: in expansion of closure expansion
/home/nmatsakis/tmp/issue-27592.rs:14:11: 14:47 note: expansion site
/home/nmatsakis/tmp/issue-27592.rs:14:5: 14:48 note: reference must be valid for the call at 14:4...
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/nmatsakis/tmp/issue-27592.rs:14:14: 14:47 note: ...but borrowed value is only valid for the block at 14:13
/home/nmatsakis/tmp/issue-27592.rs:14     write(|| format_args!("{}", "Hello world"));
                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
