
fn foo<'a>() -> &'a int {  //~ ERROR unconstrained region
    return &x;
}
static x: int = 5;
fn main() {}
