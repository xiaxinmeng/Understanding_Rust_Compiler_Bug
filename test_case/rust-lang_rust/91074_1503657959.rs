rust
pub trait MyTrait {
    fn func(&self, arg: &str) -> bool {
        let _unused_args_for_rustdoc = arg;
        false
    }
}
