rust
fn foo((_x, _): (LogDrop, LogDrop), (_, _y): (LogDrop, LogDrop)) {
// for naming:
// fn foo((a, b): (LogDrop, LogDrop), (c, d): (LogDrop, LogDrop)) {
// variables dropped "immediately," before function runs;
// technically I think c should drop first to preserve "reverse of declaration order"
// c dropped
// b dropped
// function runs
// variables dropped in reverse of declaration order:
// d dropped
// a dropped
}
