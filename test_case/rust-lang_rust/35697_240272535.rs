
ImplItemKind::Method(ref sig, _) => {
    // If this is a trait impl, ensure the method
    // exists in trait
    this.check_trait_item(impl_item.ident.name,
        impl_item.span,
        |n, s| ResolutionError::MethodNotMemberOfTrait(n, s));
