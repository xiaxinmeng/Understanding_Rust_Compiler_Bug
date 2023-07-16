 rust
let mut tmp = foo();
let mut tmp = foo.bar();
{
    let mut tmp2 = bip();
    let mut tmp2 = tmp2.bop();
    let mut tmp2 = tmp2.boop();
    let mut tmp2 = &tmp2;
    tmp.baz(tmp2)
}
