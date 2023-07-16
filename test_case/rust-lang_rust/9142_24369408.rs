
let map1: TreeMap<~str,~str> = TreeMap::new();
...
let encoded = io::with_str_writer |wr| {
    let mut encoder = json::Encoder(wr);
    map1.encode(&mut encoder);
};
let mut decoder = Decoder::decode(json::from_str(encoded).unwrap());
let map2: TreeMap<~str, ~str> = Decodable::decode(&mut decoder); 
