rust
fn static_table() -> &'static mut [u8; 20] {
    let ref mut table = [0; 20];
    table
}

fn main() {
    let table = static_table();
    table[4] = 20;
    println!("{:?}", table);
}
