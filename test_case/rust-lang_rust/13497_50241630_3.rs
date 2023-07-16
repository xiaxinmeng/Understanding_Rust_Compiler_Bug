
$ rustc r13497.rs 
r13497.rs:3:5: 3:13 error: `rawLines` does not live long enough
r13497.rs:3     rawLines.iter().map(|l| l.as_slice().trim()).collect()
                ^~~~~~~~
r13497.rs:1:46: 4:2 note: reference must be valid for the lifetime 'a as defined on the block at 1:45...
r13497.rs:1 fn read_lines_borrowed<'a>() -> Vec<&'a str> {
r13497.rs:2     let rawLines: Vec<String> = vec!["foo  ".to_string(), "  bar".to_string()];
r13497.rs:3     rawLines.iter().map(|l| l.as_slice().trim()).collect()
r13497.rs:4 }
r13497.rs:1:46: 4:2 note: ...but borrowed value is only valid for the block at 1:45
r13497.rs:1 fn read_lines_borrowed<'a>() -> Vec<&'a str> {
r13497.rs:2     let rawLines: Vec<String> = vec!["foo  ".to_string(), "  bar".to_string()];
r13497.rs:3     rawLines.iter().map(|l| l.as_slice().trim()).collect()
r13497.rs:4 }
error: aborting due to previous error
