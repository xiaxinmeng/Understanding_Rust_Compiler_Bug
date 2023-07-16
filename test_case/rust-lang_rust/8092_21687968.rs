
$ cat foo.rs
struct A;
impl Drop for A {
    fn drop(&self) {
        println("drop");
    }
}

fn main() {
    let a = @A;
    std::util::ignore(a);
    println("here");
}
$ rust run foo.rs
warning: no debug symbols in executable (-arch x86_64)
here
drop
