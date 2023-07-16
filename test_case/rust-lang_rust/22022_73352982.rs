 rust
fn read_field<D, T>(d: &mut D, field: &str) -> Result<T, <D as Decoder>::Error> 
where D: Decoder, T: Decodable
