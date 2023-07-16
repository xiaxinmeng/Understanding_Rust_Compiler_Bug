rust
    for dwo in codegen_results.modules.iter().filter_map(|m| m.dwarf_object.as_ref()) {
        ab.add_file(dwo);
    }
