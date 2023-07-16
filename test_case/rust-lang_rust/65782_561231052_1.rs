rust
struct NotSend;
unsafe impl !Send for NotSend {}

let mut a: NotSend = NotSend;
yield;
