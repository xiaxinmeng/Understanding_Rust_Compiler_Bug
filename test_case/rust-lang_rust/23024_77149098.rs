 rust
fn main()
{
    fn h(x:i32) -> i32 {3*x}
    let vfnfer = vec![&h];
    let f = vfnfer[0] as Fn;
}
