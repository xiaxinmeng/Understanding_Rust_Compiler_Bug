rust
static A: i32 = {
    let p = &A;
    *p
};
