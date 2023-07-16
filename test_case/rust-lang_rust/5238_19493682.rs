
-> % rustc bar.rs
bar.rs:4:8: 4:19 error: unresolved import: found `Name` in `???` but it is private
bar.rs:4     use super::Name;
                 ^~~~~~~~~~~
bar.rs:4:8: 4:19 error: failed to resolve import `super::Name`
bar.rs:4     use super::Name;
                 ^~~~~~~~~~~
error: aborting due to 2 previous errors
