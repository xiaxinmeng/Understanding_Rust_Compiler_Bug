rust
fn work_with_stdio(io: Stdio) {}
var file = File::open("foo.txt").unwrap();
work_with_stdio(Stdio::from(file));
