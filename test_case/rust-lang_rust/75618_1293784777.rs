rust
pub fn sum(chars: &[char]) -> u32 {
    chars.iter().map(|&c| c as u32).sum()
}

const S: &str = "中文English🤣";

pub fn s() -> u32 {
    sum(&const_str::to_char_array!(S))
}

#[test]
fn test() {
    let chars = S.chars().collect::<Vec<_>>();
    assert_eq!(s(), sum(&chars))
}
