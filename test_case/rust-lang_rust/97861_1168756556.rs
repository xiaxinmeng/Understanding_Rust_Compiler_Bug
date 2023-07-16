rust
let (val, overflowed) = (val.into(), Scalar::from_bool(overflowed).into());
if let Abi::ScalarPair(..) = dest.layout.abi {
    self.write_immediate(Immediate::ScalarPair(val, overflowed), dest)
} else {
    let dest_val = self.place_field(&dest, 0)?;
    let dest_overflowed = self.place_field(&dest, 1)?;
    self.write_scalar(val, dest_val)?;
    self.write_scalar(overflowed, dest_overflowed)
}
