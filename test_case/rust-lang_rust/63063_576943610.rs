rust
#![feature(type_alias_impl_trait)]
pub type Parser<'a, X, T, V> = impl Fn(X) -> Result<(T, V), String>;

pub fn take_cpredicate<'a>(
    predicate: impl Fn(char) -> bool,
) -> Parser<'a, &'a str, &'a str, char,> {
    move |s| {
        if s.len() == 0 {
            return Err(s.to_string());
        }
        let mut chars = s.chars();
        let next = chars.next().unwrap();
        if predicate(next) {
            Ok((&s[1..], next))
        } else {
            Err(s.to_string())
        }
    }
}
