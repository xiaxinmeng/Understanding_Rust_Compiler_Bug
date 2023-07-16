rust
for import in determinate_imports.chain(indeterminate_imports) {
    if let Some(err) = finalize_import(import) {
        report_error(err)
    } else if is_indeterminate(import) {
        report_error("cannot determine resolution")
    }
}
