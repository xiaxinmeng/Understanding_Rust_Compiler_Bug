rust
pub fn test_3(a: [i32; 10]) -> i32 {
    a.iter().skip(8).fold(0, |sum, v| sum + v)
}
