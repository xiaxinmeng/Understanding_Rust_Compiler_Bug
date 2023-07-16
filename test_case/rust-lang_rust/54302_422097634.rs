rust
trait Deserialize<'de> {
    fn from_str(s: &'de str) -> Self;
}

trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

impl<'de: 'a, 'a> Deserialize<'de> for &'a str {
    fn from_str(s: &'de str) -> Self {
        s
    }
}

fn main() {
    println!("{}", use_after_free::<&'static str>());
}

fn use_after_free<T: DeserializeOwned>() -> T {
    let s = "oh my".to_owned();
    T::from_str(&s)
}
