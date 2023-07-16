rust
let val = match (ap, mode) {
    (ApInc, Inc(step)) => step,  // &f32
    _ => 0.,  // f32
};
