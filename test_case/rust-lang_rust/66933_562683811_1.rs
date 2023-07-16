
fn foo() {
    let v = vec![1_f64, 2.2_f64];
    let mut fft: Vec<Vec<f64>> = vec![];

    let x1: &[f64] = &v;
    let x2 = x1.into_iter().collect::<Vec<f64>>(); // <- point here
    fft.push(x2); // don't point here!
}
