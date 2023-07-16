
fn foo() {
    #[test]
    fn bar(){}
    use self::bar as bar_gensym;
}
