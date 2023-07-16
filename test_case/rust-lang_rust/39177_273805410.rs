rust
fn stream(reader: &Reader) -> Parser<u8, Stream> {
	println!("READER addr is (oside): {:p}, {:x}", reader, reader.v); // ADDED
	dictionary() - space() - seq(b"stream") - eol() >>
	|dict: Dictionary| {
		println!("READER addr is (iside): {:p}, {:x}", reader, reader.v); //ADDED
		reader.print_xref_size();
		let length = dict.get("Length").and_then(|value| {
			if let Some(id) = value.as_reference() {
				return reader.get_object(id).and_then(|value|value.as_i64());
			}
			return value.as_i64();
		}).expect("Stream Length should be an integer.");
		let stream = take(length as usize) - eol().opt() - seq(b"endstream");
		stream.map(move |data|Stream::new(dict.clone(), data))
	}
}
