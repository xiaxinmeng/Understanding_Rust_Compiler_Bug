rust
#[lang = "const_eval_select"]
pub fn const_eval_select<R, F: FnOnce() -> R, G: const FnOnce() -> R>(f: F) -> R {
    f()
}
