rust
/// let url = Url { scheme: ~"https",
///                 user: Some(UserInfo { user: ~"username", pass: None }),
///                 host: ~"example.com",
///                 port: Some(~"8080"),
///                 path: ~"/foo/bar",
///                 query: ~[(~"baz", ~"qux")],
///                 fragment: Some(~"quz") };
/// // https://username@example.com:8080/foo/bar?baz=qux#quz
/// 