 rust
let mut i = 0i;
{
    let p = &mut i;
    *p = 1;
}
{
    let q = &mut i;
    *q = 2;
}
