rust
const V: Vec<u8> = {
    let mut res = vec![];
    res.extend(V.iter());
    res
};
