rust
let _: f32 = 1. - 1.;   // allowed
let _: f32 = 1. - &1.;  // type error
let _: f32 = &1. - 1.;  // type error
let _: f32 = &1. - &1.; // type error
