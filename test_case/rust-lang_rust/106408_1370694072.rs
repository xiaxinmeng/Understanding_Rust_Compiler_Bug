rust
#[proc_macro]
pub fn foo(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
  r#"{ let x = 5; format!("{x}") }"#.parse().unwrap()
}
