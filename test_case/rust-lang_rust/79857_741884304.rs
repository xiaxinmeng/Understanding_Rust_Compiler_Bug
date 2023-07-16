rust
const BIGGER_MSG_SIZE: usize = 200 * 1024 * 1024;
const BIGGER_MSG: [u8; BIGGER_MSG_SIZE] = [2u8; BIGGER_MSG_SIZE];

fn main() {
    println!("{:?}", BIGGER_MSG[12]);
}
