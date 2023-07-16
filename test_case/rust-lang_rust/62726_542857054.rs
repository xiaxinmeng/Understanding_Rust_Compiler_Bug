rust
let mut slices = [std::mem::replace(self, IoSlice::new(&[]))];
let slices = IoSlice::advance(&mut slices[..], cnt);
match slices {
    [slice] => {
        *self = std::mem::replace(slice, IoSlice::new(&[]));
    }
    [] => {
        *self = IoSlice::new(&[]);
    }
    _ => unreachable!(),
}
