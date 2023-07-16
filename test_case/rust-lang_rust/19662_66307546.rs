
test6.rs:5:31: 5:35 help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
test6.rs:5 pub fn f(x: &int, y: &int) -> &int {
                                         ^~~~
