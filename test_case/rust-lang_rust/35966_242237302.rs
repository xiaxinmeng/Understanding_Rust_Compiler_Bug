 rust
let span = match it.node {
    ForeignItem_::ForeignItemFn(_, ref generics) => generics.span().unwrap_or(it.span),
    _ => it.span
};
