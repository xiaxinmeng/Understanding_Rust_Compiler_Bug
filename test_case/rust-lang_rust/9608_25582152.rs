
#[inline(never)]
fn idx_riced<'a, T>(v: &'a [T], i: uint) -> Option<&'a T> {
    if i < v.len() {
        unsafe {
            Some(std::cast::transmute(v.unsafe_ref(i)))
        }
    } else {
        None
    }
}

#[inline(never)]
fn idx<'a, T>(v: &'a [T], i: uint) -> Option<&'a T> {
    if i < v.len() {
        Some(&v[i])
    } else {
        None
    }
}

fn main() {
    let v = [1.0f32, 2.0, 3.0, 4.0];
    let x = idx(v, 7);
    let y = idx_riced(v, 7);
}
