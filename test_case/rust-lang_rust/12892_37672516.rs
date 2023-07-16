
$ cat foo.rs
use std::io::File;

fn main() {
    let mut buf = [0, .. 2048];
    let mut f = File::open(&Path::new("foo.rs"));
    println!("{}", f.read(buf));
}
$ rustc foo.rs
$ ./foo
Ok(149)
