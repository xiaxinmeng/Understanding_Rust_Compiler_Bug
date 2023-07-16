rust
const fn root() {
    leaf_1();
    loopy();
    leaf_2();
}
const fn loopy() {
    leaf_3();
    for _ in ... {
        leaf_4();
    }
    leaf_5();
}
const fn leaf_{1,2,...}() {
    // Does some work but relatively quickly.
}
