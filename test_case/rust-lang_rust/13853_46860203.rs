
trait Deserializer<'a> { }

trait Deserializable {
    fn deserialize_token<'a, D: Deserializer<'a>>(D, &'a str) -> Self;
}

impl<'a, T: Deserializable> Deserializable for &'a str {
    fn deserialize_token<D: Deserializer<'a>>(_x: D, _y: &'a str) -> &'a str {
    }
}

fn main() {
}
