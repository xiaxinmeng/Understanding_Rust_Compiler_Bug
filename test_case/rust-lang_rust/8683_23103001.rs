
$ cat asdf.rs
fn main() {
    printfln!("%? %?", file!(), line!());
}
$ rust run asdf.rs
warning: no debug symbols in executable (-arch x86_64)
"asdf.rs" 2
