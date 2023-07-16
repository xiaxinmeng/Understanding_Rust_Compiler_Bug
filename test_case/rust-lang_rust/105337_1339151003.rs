rust
    let x = x
        .iter_mut()
        .map(<&mut [u8]>::from) // <-- this line
        .map(foo);
