rust
> let mut compressed = rustc_metadata::METADATA_HEADER.to_vec(); 
> compressed.write_all(length).unwrap();
> FrameEncoder::new(&mut compressed).write_all(metadata.raw_data()).unwrap(); 
> 