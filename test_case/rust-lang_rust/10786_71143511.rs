
$ touch foo.rs
$ rustc --crate-type dylib,rlib foo.rs
$ ls
foo.rs       libfoo.dylib libfoo.rlib
$ echo 'pub fn foo() {}' > foo.rs
$ rustc --crate-type rlib foo.rs
$ echo 'extern crate foo; fn main() { foo::foo(); }' > bar.rs
$ rustc bar.rs -L. -C prefer-dynamic
error: linking with `cc` failed: exit code: 1
note: cc '-m64' '-L' '/Users/acrichton/code/rust/lib/rustlib/x86_64-apple-darwin/lib' '-o' 'bar' 'bar.o' '-Wl,-force_load,/Users/acrichton/code/rust/lib/rustlib/x86_64-apple-darwin/lib/libmorestack.a' '-Wl,-dead_strip' '-nodefaultlibs' '-L' '/Users/acrichton/test' '-lfoo' '-L' '/Users/acrichton/code/rust/lib/rustlib/x86_64-apple-darwin/lib' '-lstd-4e7c5e5c' '-L' '.' '-L' '/Users/acrichton/code/rust/lib/rustlib/x86_64-apple-darwin/lib' '-L' '/Users/acrichton/test/.rust/lib/x86_64-apple-darwin' '-L' '/Users/acrichton/test/lib/x86_64-apple-darwin' '-L' '/Users/acrichton/.rust/lib/x86_64-apple-darwin' '-lSystem' '-lpthread' '-lc' '-lm' '-lcompiler-rt'
note: ld: warning: directory not found for option '-L/Users/acrichton/test/.rust/lib/x86_64-apple-darwin'
ld: warning: directory not found for option '-L/Users/acrichton/test/lib/x86_64-apple-darwin'
ld: warning: directory not found for option '-L/Users/acrichton/.rust/lib/x86_64-apple-darwin'
Undefined symbols for architecture x86_64:
  "foo::ha37403d109b4e734eaa", referenced from:
      main::h7e1981c1a43d0b7afaa in bar.o
ld: symbol(s) not found for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
