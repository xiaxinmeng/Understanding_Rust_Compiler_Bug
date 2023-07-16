rust

struct ExactMatchString<const S: &'a str>;

impl<const S: &'a str> MatchString for ExactMatchString<S> {
    fn match_string(string: &str) -> Option<darling::Result<Self>> {
        if string == S {
            Some(Ok(ExactMatchString))
        } else {
            None
        }
    }
}
