
enum CString {
    Empty([u8; 1]),
    Some(Box<[u8]>),
}
