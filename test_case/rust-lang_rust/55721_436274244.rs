rust
const FOO : [i32;1] = [0];

fn main() {
    let mut tmp = FOO;
    tmp[0] = 1;
    println!("{:?}",FOO);
}
