rust
extern crate proc_macro;

#[proc_macro_attribute]
pub fn first(
  _attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  item
}

#[proc_macro_attribute]
pub fn second(
  _attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  // Note: if I return `item` as-is (same as `first` above), it resolves the issue!
  let mut out: proc_macro::TokenStream = proc_macro::TokenStream::new();
  out.extend(item);
  out
}
