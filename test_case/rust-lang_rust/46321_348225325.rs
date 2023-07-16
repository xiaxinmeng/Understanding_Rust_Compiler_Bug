rust
let (a, b, x);
x = None;
loop {
    a = init;
    use(x, &mut a);
    endregion('α);
    b = move a;
    x = Some(&'α b);
}
endregion('α);
