rust
pub struct Header {
    pub name: * const u8,
    pub name_len: usize,
    pub value: * const u8,
    pub value_len: usize,
}

pub struct HeaderIndices {
    name_a :usize,
    name_b: usize,
    value_a: usize,
    value_b: usize,
}


pub fn record_header_indices(bytes_ptr: usize, headers: * const Header, indices: *mut HeaderIndices, len: isize) {
    for i in 0 .. len {
        let mut indices = unsafe { &mut *indices.offset(i) };
        let header = unsafe { &*headers.offset(i) };
        let name_start = header.name as usize - bytes_ptr;
        let name_end = name_start + header.name_len;
        indices.name_a = name_start;
        indices.name_b = name_end;
        let value_start = header.value as usize - bytes_ptr;
        let value_end = value_start + header.value_len;
        indices.value_a = value_start;
        indices.value_b = value_end;
    }
}
