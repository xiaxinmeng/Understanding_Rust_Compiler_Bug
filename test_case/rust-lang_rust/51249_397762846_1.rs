
[00:52:06] 18   --> $DIR/enum.rs:29:9
[00:52:06] 19    |
[00:52:06] 19    |
[00:52:06] 20 LL |     while let Some(x) = &Some(3) {
[00:52:06] -    |                    - consider changing this to `x`
[00:52:06] +    |                    - help: use a mutable reference instead: `x`
[00:52:06] 22 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:52:06] 23    |         ^^^^^^^ cannot borrow as mutable
