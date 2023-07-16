
let a = HashMap::new();
a.insert(1, 2);
a.insert(3, 4);
let a_bytes = io::with_bytes_writer(|wr| a.encode(&BytesEncoder::new(&wr)));

let b = HashMap::new();
b.insert(3, 4);
b.insert(1, 2);
let b_bytes = io::with_bytes_writer(|wr| b.encode(&BytesEncoder::new(&wr)));
