console
$ cat > throw.cpp
extern "C" { void may_throw() { throw "foo"; } }
$ clang -c throw.cpp -o throw.o
$ ar q libthrow.a throw.o
$ cat > catch.rs
#![feature(c_unwind)]

extern "C-unwind" {
    fn may_throw();
}

fn main() {
    unsafe { may_throw(); }
}
$ rustc +nightly-2022-08-30 catch.rs -l throw -L . -l c++
$ ./catch
fatal runtime error: Rust cannot catch foreign exceptions
zsh: abort      ./catch
