rust
use ::std::ptr::NotNull;
use ::core::nonzero::Nonzero;

fn foo() {
    let raw = NotNull::new(&0).unwrap();
    let _ = Box::from_not_null_raw(raw);
}
