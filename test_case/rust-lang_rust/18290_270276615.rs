rust
const ACT_STRINGS: &'static [&'static str] = &["set name=foo value=foo"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        for astr in ACT_STRINGS.iter() {
            println!("{}", astr);
        }
    }   
}
