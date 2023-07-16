rust
// ICE everywhere.
struct S;
impl S {
    #[derive(Debug)]
    fn f() {
        file!(); // must invoke a macro.
    }
}
