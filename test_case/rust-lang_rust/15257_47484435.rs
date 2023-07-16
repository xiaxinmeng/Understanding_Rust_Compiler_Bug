
pub fn require_unique_names(diagnostic: &SpanHandler, metas: &[Gc<MetaItem>]) {
    let mut set: HashSet<InternedString> = HashSet::new();
    for meta in metas.iter() {
        let name = meta.name();

        if !set.insert(name.clone()) {
            diagnostic.span_fatal(meta.span,
                                  format!("duplicate meta item `{}`",
                                          name).as_slice());
        }
    }
}
