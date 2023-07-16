rust
pub struct Header<'a> {
    pub name: &'a [u8],
    pub value: &'a [u8],
}

pub struct HeaderIndices {
    name: (usize, usize),
    value: (usize, usize),
}


pub fn record_header_indices(bytes: &[u8], headers: &[Header], indices: &mut [HeaderIndices]) {
    let bytes_ptr = bytes.as_ptr() as usize;
    for (header, indices) in headers.iter().zip(indices.iter_mut()) {
        let name_start = header.name.as_ptr() as usize - bytes_ptr;
        let name_end = name_start + header.name.len();
        indices.name = (name_start, name_end);
        let value_start = header.value.as_ptr() as usize - bytes_ptr;
        let value_end = value_start + header.value.len();
        indices.value = (value_start, value_end);
    }
}
