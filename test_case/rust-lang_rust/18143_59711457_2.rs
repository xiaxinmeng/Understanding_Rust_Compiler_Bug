 rust
#![crate_type="lib"]

extern {
    fn black(_: *const u8);
}

pub unsafe fn test() {
    let data = std::io::stdin().read_to_end().unwrap();
    let mut end = data.as_ptr().offset(data.len() as int - 1);
    let mut cur = end;
    loop {
        while *cur != b'>' {
            cur = cur.offset(-1);
        }

        black(cur);
    }
}
