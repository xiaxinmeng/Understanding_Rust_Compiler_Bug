 Rust
fn get_debug_str(s: &str) -> impl fmt::Debug {
    s
}

fn get_debug_string(s: &str) -> impl fmt::Debug {
    s.to_string()
}

fn good(s: &str) -> Box<fmt::Debug+'static> {
    // if this does not compile, that would be quite annoying
    Box::new(get_debug_string())
}

fn bad(s: &str) -> Box<fmt::Debug+'static> {
    // if this *does* compile, we have a problem
    Box::new(get_debug_str())
}
