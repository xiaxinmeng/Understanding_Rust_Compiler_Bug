rust
#[cfg(test)]
mod tests {
    #[test]
    fn closure_lifetimes() {
        let input: String = String::from("hello world");
        let closure = |s: &str| -> &str {&s[..]};
        let output: &str = closure(&input);
        assert_eq!(input, output);
    }
}
