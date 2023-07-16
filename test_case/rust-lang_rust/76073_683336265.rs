
$ RUSTDOC_LOG=rustdoc::passes::collect=debug rustdoc +stage1 pub-use.rs 
2020-08-29T20:03:27.412092Z DEBUG rustdoc::passes::collect_intra_doc_links: attempting to resolve item without parent module: std::fs
warning: unresolved link to `std::fs`
 --> pub-use.rs:1:6
  |
1 | /// [`std::fs`]
  |      ^^^^^^^^^ unresolved link
