rust
    to.extend(from.as_ref().iter().map(|value| -> &dyn Debug { value }));
