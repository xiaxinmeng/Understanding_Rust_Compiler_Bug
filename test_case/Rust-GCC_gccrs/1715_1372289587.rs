c++
for (const auto &attr: attrs) {
    switch (attr.kind) {
        case Attr::Inline: handle_inline_attribute_on_fndecl(fndecl, attr);
        case Attr::MustUse: handle_must_use_attribute_on_fndecl(fndecl, attr);
        /* ... */
    }
} 
