rust
fn main() {
    let slice1: &[i32] = &[1, 2, 3, 4, 5][..];

    let slice2 = (|x| x)(slice1);

    fun(slice1)
}

fn fun<'a>(slice1: &'a [i32]) {
    let slice3 = (|x: &'a [i32]| x)(slice1);
}
