rust
fn into_call_site_span(stream: TokenStream) -> TokenStream {
    let call_site_span = Span::call_site();
    let iter = stream.into_iter().map(|mut tree| {
        match tree {
            TokenTree::Group(g) => {
                TokenTree::Group(Group::new(g.delimiter(), into_call_site_span(g.stream())))
            }
            _ => {
                tree.set_span(call_site_span);
                tree
            }
        }
    });
    TokenStream::from_iter(iter)
}
