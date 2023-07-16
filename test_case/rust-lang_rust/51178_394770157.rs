rust
// & <-> Owned are comparable
impl<'a> PartialEq<&'a str> for String
impl<'a> PartialEq<String> for &'a str

// utf8 <-> OS are comparable
impl PartialEq<str> for OsString
impl PartialEq<OsString> for str
