rust
struct Foo<
    T, 
    const N: usize, 
    U, 
    const M: usize = 3
    V = [i64; M],
>(T, U, V);

fn foo<const N: usize, T, const M: usize>() { }
