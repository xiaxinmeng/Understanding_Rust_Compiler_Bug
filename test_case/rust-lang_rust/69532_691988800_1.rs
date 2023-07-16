rust
let src_nan_mask = !(Self::QNAN.significand() | Self::SNAN.significand());
let dst_nan_mask = !(T::QNAN.significand() | T::SNAN.significand());

let unmasked_payload = source.significand() & src_nan_mask;
let dest_mask = T::largest() & dst_nan_mask;
let new_payload = unmasked_payload & dest_mask;

let mut nan = if source.is_snan() {
    T::snan(Some(new_payload))
} else {
    T::qnan(Some(new_payload))
};
nan.sign = source.sign;
return nan;
