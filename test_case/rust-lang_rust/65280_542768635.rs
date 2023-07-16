rust
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_list()
        .entries(backtrace.frames.iter().map(|f| DebugFrame(f)))
        .finish()
}
