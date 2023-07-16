
$ echo 'fn main() { println!("{:?}", std::env::current_exe()); }' > foo.rs
$ rustc foo.rs
$ ./foo
Ok("/home/alex/foo")
$ ln foo bar
$ ./bar
Ok("/home/alex/bar")
