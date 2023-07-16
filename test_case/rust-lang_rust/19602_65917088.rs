
fn main() {
     let re = regex!("match me");
     assert!(re.as_str().char_at(0), '^')
}
