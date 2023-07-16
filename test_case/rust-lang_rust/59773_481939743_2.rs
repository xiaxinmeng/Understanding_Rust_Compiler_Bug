
    let string = b"dfasdfas";
    let mut target = [b'\0'; 8];
    b.iter(|| {
        for (idx, i) in string.iter().enumerate() {
            target[idx] = *i;
        }
    });
