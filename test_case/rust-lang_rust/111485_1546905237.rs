rust
fn main() {
    indoc::formatdoc! {
        r#"abcabcabcabcabcabcabcab
           abcde
           abcdefghij
           abc {cc}
        "#,
        cc = "".to_string(),
    };
}
