
let bytes = {
    let len = names_bytes as uint - 1;
    let mut bytes = slice::with_capacity(len);
    try!(file.push_at_least(&mut bytes, len, len));
    bytes
};
