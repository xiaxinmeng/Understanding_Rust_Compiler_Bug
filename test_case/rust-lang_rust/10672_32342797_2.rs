
fn swap<T>(mut_a: &mut T, mut_b: &mut T) {
    let mut x: T;
    let out_x = &out x;
    // `take_then_put_back` taking a once fn is important so we can move `out_x` into it
    let out_b = take_then_put_back(mut_b, |move_b| *out_x = *move_b);
    let out_a = take_then_put_back(mut_a, |move_a| *out_b = *move_a);
    *out_a = x;
}
