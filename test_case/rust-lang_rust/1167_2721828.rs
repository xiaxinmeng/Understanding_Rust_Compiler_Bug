
fn p<T>(t: t<T>, ofs: uint) -> *mutable T {
    // I'd like to do the following, but FIXME #1173
    // (*t).base + (ofs as *T)
    ((*t).base as uint + ofs) as *mutable T
}
