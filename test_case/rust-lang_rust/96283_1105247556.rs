rust
let array = [...];

// Steal metadata from an example or make the metadata from whole cloth
let dst = &array[something];
let metadata = get_metadata(dst);

// Now build a raw DST from a pointer to the whole array
let raw_array_ptr = array.as_ptr();
let raw_dst = ptr::from_raw_parts(raw_array_ptr, metadata);

// Yay this actually can be strided through the whole array now
let next_dst = raw_dst.byte_add(offset_to_next_dst);
