 rust
pub fn main() {
    println!("{:?}", "In Chinese: 文件不存在");
    // still prints: "In Chinese: \u{6587}\u{4ef6}\u{4e0d}\u{5b58}\u{5728}"
    // expected: "In Chinese: 文件不存在"
}
