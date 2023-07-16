rust

type MatchFooLitStr = MatchLitStr<ExactMatchString<"foo">>;

fn read_foo(meta: &syn::NestedMeta) {
    if let Some(Ok(MatchFooLitStr(_))) = MatchFooLitStr::match_nested_meta(meta) {
        // ...
    }
}

