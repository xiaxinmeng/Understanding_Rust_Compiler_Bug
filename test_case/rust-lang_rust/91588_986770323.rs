rust
fn select<'a, const N: usize>(data: &'a mut [u32; N],
                              keep: &mut [bool; N]) -> &'a [u32] {
    let mut pos = 0;
    for (i, &b) in keep.iter().enumerate() {
        if b {
            data[pos] = data[i];
            pos += 1;
        }
    }
    &data[.. pos]
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let lives_longer_than_keep = {
           let mut keep = [true, false, true, false, true];
           select(&mut data, &mut keep)
    };
    println!("{:?}", lives_longer_than_keep);
}
