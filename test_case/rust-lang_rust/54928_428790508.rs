rust
use itertools::Itertools;
fn fn_slice_method_4(x: &[f32], out: &mut [f32]) {
    assert_eq!(x.len(), out.len());
    let mut it = x.iter().tuples();
    out.iter_mut().tuples().for_each(|(a0, a1)| {
        let (b0, b1) = it.next().unwrap();
        *a0 = *b1 * 123.0;
        *a1 = *b0 * 456.0;
    });
}
