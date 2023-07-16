rust
NativeLibKind::Static { bundle: Some(false), whole_archive } => {
    // Link "static-nobundle" native libs only if the crate they originate from
    // is being linked statically to the current crate.  If it's linked dynamically
    // or is an rlib already included via some other dylib crate, the symbols from
    // native libs will have already been included in that dylib.
    if data[cnum.as_usize() - 1] == Linkage::Static {
        if whole_archive == Some(true) {
            cmd.link_whole_staticlib(name, verbatim, &search_path);
        } else {
            cmd.link_staticlib(name, verbatim)
        }
    }
}
