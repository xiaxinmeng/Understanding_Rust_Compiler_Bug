rust
let ptr = make_weird_raw_ptr();
let meta = metadata(ptr);
let size = meta.size(); // *oops* UB
