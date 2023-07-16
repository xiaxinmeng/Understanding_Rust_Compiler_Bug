 rust
fn main() {
    let i = vec!["hello", "there", "yo"];
    let mut bytes = Vec::from_elem(1000, 0);
    {
        // We need to make the temp variable so that borrowck "gets" what we're doing.
        let mut temp = bytes.as_mut_slice();
        for s in i.iter() {
            let bytes = temp;
            std::slice::bytes::copy_memory(bytes, s.as_bytes());
            bytes[s.len()] = 0;
            temp = bytes.slice_from_mut(s.len() + 1);
        }
    }
}

