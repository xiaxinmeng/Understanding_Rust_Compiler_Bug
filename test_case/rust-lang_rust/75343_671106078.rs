rust
trait Deserialize<'de> {
    fn deserialize();
}

struct Container<T: Deserialize> {
    data: T
}

impl <'de, T: Deserialize> Deserialize<'de> for Container<T> where T: Deserialize<'de> {
    fn deserialize() { 
        loop {}
    } 
}
