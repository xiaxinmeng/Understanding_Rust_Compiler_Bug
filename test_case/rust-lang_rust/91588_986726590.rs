rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let lives_longer_than_keep = {
           let mut keep = [true, false, true, false, true];
           select(&mut data, &mut keep)
    };
    println!("{:?}", lives_longer_than_keep);
}
