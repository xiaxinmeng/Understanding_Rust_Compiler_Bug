 rust
fn f() -> f64 { // Ok
   0.0f64 +  vec![1.0f64].into_iter().sum::<f64>()
}

fn f2() -> f64 { // Error
   0.0f64 +  vec![1.0f64].into_iter().sum()
}
