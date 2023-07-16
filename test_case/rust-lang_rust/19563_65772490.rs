 rust
pub fn encode_metadata(parms: EncodeParams, krate: &ast::Crate) -> Vec<u8> {
    let mut wr = SeekableMemWriter::new();

    // Reserve 4 bytes for the length.
    wr.write_be_u32(0).unwrap();

    encode_metadata_inner(&mut wr, parms, krate);

    // Now go back to the beginning and fill the length.
    let len = (wr.tell().unwrap() - 4).to_u32().unwrap();
    wr.seek(0, SeekStyle::SeekSet).unwrap();
    wr.write_be_u32(len).unwrap();

    wr.unwrap()
}
