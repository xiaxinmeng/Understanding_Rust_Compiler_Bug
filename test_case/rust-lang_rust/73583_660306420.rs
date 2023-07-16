rust
use std::path::Path;
pub fn main() {
    dbg!("foo-bar" < "foo/bar");
    dbg!(Path::new("foo-bar") < Path::new("foo/bar"));
    dbg!(vec!["foo-bar"] < vec!["foo", "bar"]);
}
