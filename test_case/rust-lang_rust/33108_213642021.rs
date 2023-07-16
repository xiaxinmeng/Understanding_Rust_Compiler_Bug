 rust
fn foo<T: some_mod::SomeTrait>(x: &T) {
    /* now I can call methods of SomeTrait, even thogh it is not imported into scope */
}
