rust
fn extract_tupled_types(attr: Attribute) -> Result<Vec<Type>> {
    let ty: Type = syn::parse2(attr.tts)?;
    Ok(match ty {
        Type::Paren(paren) => vec![*paren.elem],
        Type::Tuple(tuple) => tuple.elems.into_iter().collect(),
        _ => vec![ty],
})
