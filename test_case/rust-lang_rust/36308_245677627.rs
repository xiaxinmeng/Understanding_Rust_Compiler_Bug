 rust
let input_span = Span {
    expn_id: ecx.codemap().record_expansion(ExpnInfo {
        call_site: span,
        callee: NameAndSpan {
            format: MacroAttribute(intern(&pprust::meta_item_to_string(meta_item))),
            span: Some(span),
            allow_internal_unstable: true,
        },
    }),
    ..item.span
};
