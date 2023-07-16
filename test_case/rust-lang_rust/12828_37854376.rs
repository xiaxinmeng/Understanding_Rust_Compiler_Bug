 rust
pub fn buffer_encode<T:Encodable<Encoder<'a>>>(to_encode_object: &T) -> ~[u8] {
   //Serialize the object in a string using a writer
    let mut m = MemWriter::new();
    {
        let mut encoder = Encoder::new(&mut m as &mut io::Writer); // `m` does not live long enough
        to_encode_object.encode(&mut encoder);
    }
    m.unwrap()
}
