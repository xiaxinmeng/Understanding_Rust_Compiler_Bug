
fn reverse(xs: &mut [i32]) {
    xs.reverse()
}

fn do_nothing(xs: &mut [i32]) {
    reverse(xs);
    reverse(xs);
}
