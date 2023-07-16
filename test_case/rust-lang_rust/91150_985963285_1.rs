rust
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn repro(_input: TokenStream) -> TokenStream {
    // type T = <str as ⟪ToOwned⟫>::Owned;
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("type", Span::call_site())),
        TokenTree::Ident(Ident::new("T", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Punct(Punct::new('<', Spacing::Alone)),
        TokenTree::Ident(Ident::new("str", Span::call_site())),
        TokenTree::Ident(Ident::new("as", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::None, TokenStream::from_iter([
            TokenTree::Ident(Ident::new("ToOwned", Span::call_site()))
        ]))),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Owned", Span::call_site())),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ])
}
