rust
pub trait MatchNestedMeta: Sized {
    fn match_nested_meta(meta: &syn::NestedMeta) -> Option<darling::Result<Self>>;
}

pub trait MatchString: Sized {
    fn match_string(str: &str) -> Option<darling::Result<Self>>;
}

impl MatchString for String {
    fn match_string(string: &str) -> Option<darling::Result<Self>> {
        Some(Ok(string.to_string()))
    }
}

pub struct MatchLitStr<S: MatchString = String>(pub S, pub proc_macro2::Span);

impl<S: MatchString> MatchNestedMeta for MatchLitStr<S> {
    fn match_nested_meta(meta: &syn::NestedMeta) -> Option<darling::Result<Self>> {
        match meta {
            syn::NestedMeta::Lit(syn::Lit::Str(string)) => {
                S::match_string(string.value().as_str()).map(|s| Ok(MatchLitStr(s?, string.span())))
            }
            _ => None,
        }
    }
}

