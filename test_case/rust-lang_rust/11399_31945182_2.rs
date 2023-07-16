 rust
#[lang="trace_traceable"]
fn trace_traceable<T: Trace>(start: *(), end: *(), gc_info: &mut GcInfo) {
    unsafe { (*(start as *T)).trace(gc_info) }
}

#[lang="trace_nontraceable"]
fn trace_nontraceable<T>(start: *(), end: *(), gc_info: &mut GcInfo) {
    gc_info.conservative_scan(start, end)
}
