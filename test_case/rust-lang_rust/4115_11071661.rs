
fn main() {
    let map = LinearMap();
    map.insert(~"a", 1);
    map.insert(~"b", 2);
    map.insert(~"c", 3);
    map.serialize(&json::Serializer(io::stdout()));
}
