rust
fn once<S, T, U, F: FnOnce(S, T) -> U>(f: F, s: S, t: T) -> U {
    f(s, t)
}

pub fn dangle() -> &'static [i32] {
    let other_local_arr = [0, 2, 4];
    let local_arr = other_local_arr;
    let mut out: &mut &'static [i32] = &mut (&[1] as _);
    once(|mut z: &[i32], mut out_val: &mut &[i32]| {
        z = &local_arr; // Comment out this line for an error!
        *out_val = &local_arr;
    }, &[] as &[_], &mut *out);
    *out
}

fn main() {
    println!("{:?}", dangle());
}
