rust
#[test]
fn test_turkish_case() {
    let lowercase = "abcçdefgğhıijklmnoöprsştuüvyz".chars();
    let mut uppercase = "ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ".chars();
    for lower in lowercase {
        let upper = uppercase.next().unwrap();
        assert_eq!(lower.to_string().to_locale_uppercase("tr-TR"), upper.to_string());
    }
}
