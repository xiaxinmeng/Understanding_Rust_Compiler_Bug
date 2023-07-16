rust
let crate_entry = crate_root
    .index
    .lookup(metadata.raw_bytes(), CRATE_DEF_INDEX)
    .unwrap()
    .decode(&metadata);

let crate_attrs = crate_entry
    .attributes
    .decode((&metadata, self.sess))
    .collect();
