rust
pub fn f() -> () {
    const C: [S2; 1] = [S2];
    let _ = S1(C[0]).clone();
}

#[derive(Clone)]
pub struct S1(S2);

#[derive(Clone)]
pub struct S2;

