
$ cat hello.rs 
extern mod std;

use std::io::println;

fn main() {
  println("Hello World!");
}

$ rustc hello.rs 
warning: no debug symbols in executable (-arch x86_64)

$ ./hello
Hello World!
