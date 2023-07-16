
hello.rs:7:5: 9:6 error: method `next` has an incompatible type for trait: expected bound lifetime parameter , found concrete lifetime [E0053]
hello.rs:7     fn next(&'a mut self) -> Option<&'a str> {
hello.rs:8         Some("hi")
hello.rs:9     }
error: aborting due to previous error
