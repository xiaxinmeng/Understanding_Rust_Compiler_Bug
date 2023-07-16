
fn main() {
    let val = &mut vec!(1);
    let slice1 = val.as_slice(); // OK
    let slice2 = &val[]; // er
}
