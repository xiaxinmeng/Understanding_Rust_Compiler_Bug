rust
fn assert_unpin<T: std::marker::Unpin>() {}

pub fn foo() {
    assert_unpin::<Vec<syn::GenericParam>>();
}
