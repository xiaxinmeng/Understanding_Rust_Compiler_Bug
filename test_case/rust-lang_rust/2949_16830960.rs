
fn main()
{
    let mut a = 3;
    let mut x = &mut a;
    assert!(*x == 3);
    *x = 4;
    assert!(*x == 4);
    let a = 3;
    x = &a;
    assert!(*x == 5);
}
