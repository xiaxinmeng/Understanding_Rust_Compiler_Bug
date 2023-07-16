rust
    inline::record_extern_fqn(cx, did, kind);
    if let TypeKind::Trait = kind {
        inline::record_extern_trait(cx, did);
    }
