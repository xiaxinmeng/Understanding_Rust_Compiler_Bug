
struct Completions {
    mut len: libc::size_t,
}

extern mod c {
    fn linenoiseAddCompletion(lc: *Completions);
}
