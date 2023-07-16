
let pixel_ptr: *const u8 = match pixels {
     image_decoding::DecodingResult::U8(ref v) => v.as_ptr(),
     image_decoding::DecodingResult::U16(ref v) => v.as_ptr() as _,
};
