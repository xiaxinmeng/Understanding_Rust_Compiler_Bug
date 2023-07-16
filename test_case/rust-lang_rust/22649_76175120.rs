 rust
fn main()
{
    let rs = "toto";
    let s = rs.to_string();

    // That works as expected, &s coerces s to type &str
    fn titi(s:&str) { assert_eq!(s,"toto");};
    titi(&s);

    // That also works, same coercion happens
    let cs:&str = &s;
    assert_eq!(cs,rs);

    // The following expression fails to run, as though the coercion did not happen
    let cs = &s;
    assert_eq!(cs,rs);
}
