rust
struct MatchFooString;

impl MatchString for MatchFooString {
    fn match_string(string: &str) -> Option<darling::Result<Self>> {
        if string == "foo" {
            Some(Ok(MatchFooString))
        } else {
            None
        }
    }
}

