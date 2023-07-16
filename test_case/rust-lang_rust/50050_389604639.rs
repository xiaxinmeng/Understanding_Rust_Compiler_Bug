rust
let code: &str = /* some Rust code with an identifier */;
let stream: proc_macro::TokenStream = code.parse().unwrap();
let stream: proc_macro::TokenStream = stream.into_iter()
    .map(|mut tree| { tree.set_span(Span::call_site()); tree })
    .collect();
