rust
static A: u32 = 42;
static B: u32 = foo(&A); // success!

const fn foo(a: &u32) -> u32 {
    *a
}
