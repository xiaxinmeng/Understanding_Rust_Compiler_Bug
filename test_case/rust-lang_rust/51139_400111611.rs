rust
fn Qey(_1: Q) -> Qey<Q>{
    let mut _0: Qey<Q>;                  // return place

    bb0: {                              
        (_0.0: Q) = move _1;             // bb0[0]: scope 0 at src/main.rs:1:1: 1:26
        return;                          // bb0[1]: scope 0 at src/main.rs:1:1: 1:26
    }
}
