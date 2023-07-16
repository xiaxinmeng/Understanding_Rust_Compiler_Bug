rust
fn parse_data(data: &[u8]) -> u32 {
    match data {
        b"" => 1,
    }
}

fn main() {
    let val = parse_data(b"oops");
    
    println!("{}", val);
}
