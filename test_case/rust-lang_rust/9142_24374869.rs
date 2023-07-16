
let mut map1: TreeMap<~str,~str> = TreeMap::new();
map1.insert(~"k", ~"v");
map1.insert(~"foo", ~"bar");
let encoded = do io::with_str_writer |wr| {
    let mut encoder = json::Encoder(wr);
    map1.encode(&mut encoder);
};
let mut decoder = json::Decoder(json::from_str(encoded).unwrap());
let map2: TreeMap<~str, ~str> = Decodable::decode(&mut decoder);
