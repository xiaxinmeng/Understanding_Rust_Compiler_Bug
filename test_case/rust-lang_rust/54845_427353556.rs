rust
fn main() {
    let utf8: Vec<u8> = vec![237, 166, 164];

    let utf8_b: Vec<u8> = vec![
        0b1110_1101,
        0b10_100110,
        0b10_100100,
    ];
    
    {
        let codepoint = 0b1101_100110_100100;
        println!("U+{:X}", codepoint);
    }
    
    assert_eq!(utf8, utf8_b);
    
    println!("{:?}", String::from_utf8(utf8));
}
