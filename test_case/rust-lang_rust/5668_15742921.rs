
macro_rules! f_decl {
    () => (fn f ();)
}

extern {
    f_decl!();

}
