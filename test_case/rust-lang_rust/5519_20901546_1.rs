
foo.rs:1:13: 1:39 error: macros cannot expand to foreign items
foo.rs:1 pub extern { macro_rules! foo(() => ()); }
                      ^~~~~~~~~~~~~~~~~~~~~~~~~~
