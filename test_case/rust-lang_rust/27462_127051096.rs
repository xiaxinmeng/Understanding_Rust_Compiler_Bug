
let mut buf: [u8; 4096] = [0u8; 4096];

loop {
    match stream.read(&mut buf) {
        Ok(count) => {
            // handle good read
        },
        Err(e) => {
            // handle bad read
        },
    }

    // need to wipe out contents of buf, I have been doing a dumb loop equiv of memset(buf, 0, count)
}
