 rust
/// Error returned from `String::from`
#[unstable(feature = "str_parse_error", reason = "may want to be replaced with \
                                                  Void if it ever exists",
           issue = "27734")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParseError(());

#[stable(feature = "rust1", since = "1.0.0")]
impl FromStr for String {
    type Err = ParseError;
    #[inline]
    fn from_str(s: &str) -> Result<String, ParseError> {
        Ok(String::from(s))
    }
}
