
$ cat foo.rs                                           
fn main() {
  print(fmt!("%?", (-0.5f).sqrt().is_NaN()));
}
$ rustc -Z jit foo.rs                                  
true
$ rustc -O -Z jit foo.rs                               
false
$ rust run foo.rs                                     
warning: no debug symbols in executable (-arch x86_64)
true
$ rustc -O foo.rs
warning: no debug symbols in executable (-arch x86_64)
$ ./foo
false
