rust
let mut iovecs = [
    IoSlice::new(header),
    IoSlice::new(x),
];
w.write_all_vectored(&iovecs)?;
iovecs[1] = IoSlice::new(y);
w.write_all_vectored(&iovecs)?;
