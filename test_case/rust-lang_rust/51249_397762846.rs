
[00:52:06] 4 LL |     let Wrap(x) = &Wrap(3);
[00:52:06] -    |              - consider changing this to `x`
[00:52:06] +    |              - help: use a mutable reference instead: `x`
[00:52:06] 6 LL |     *x += 1; //~ ERROR cannot assign to immutable
[00:52:06] 7    |     ^^^^^^^ cannot borrow as mutable
