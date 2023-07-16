
mod external;

fn main() {
    external::hello();
    let mut bytes = vec![];
    bytes.append(&mut include_bytes!("external.rs").to_vec());
    println!("Byte count: {}", bytes.len());
}
