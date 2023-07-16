
rprichard@ryan:~/mess$ rustc format-template.rs 
format-template.rs:2:42: 2:45 error: named argument never used
format-template.rs:2     ($tmp:tt) => (format!($tmp, a=123, b=456))
                                                              ^~~
format-template.rs:2:42: 2:45 error: named argument never used
format-template.rs:2     ($tmp:tt) => (format!($tmp, a=123, b=456))
                                                              ^~~
error: aborting due to 2 previous errors
