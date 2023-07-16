rust
// ⚠️ !HACKY & PROBABLY WRONG FIX! ⚠️
if r.category() == Category::NaN {
    // this may or may not totally break the idea of arbitrary precision.
    fn significand<T: Semantics>(s: IeeeFloat<T>) -> Limb {
        // mask away int bit which is probably not relevant a lot of the time idk.
        s.sig[0] & !(1 << (T::PRECISION - 1))
    }
    let src_nan_mask = !(
        significand(Self::qnan(None)) |
        significand(Self::snan(None))
    );
    let dst_nan_mask = !(
        significand(<IeeeFloat<T>>::qnan(None)) |
        significand(<IeeeFloat<T>>::snan(None))
    );

    let unmasked_payload = significand(r) & src_nan_mask;
    let dest_mask = significand(<IeeeFloat<T>>::largest()) & dst_nan_mask;
    let new_payload = unmasked_payload & dest_mask;
    if new_payload != unmasked_payload {
        *loses_info = true;
    };

    let mut nan = if r.is_signaling() {
        <IeeeFloat<T>>::snan(Some(new_payload))
    } else {
        <IeeeFloat<T>>::qnan(Some(new_payload))
    };
    nan.sign = r.sign;
    // FIXME: THERE'S A BUNCH OF NAN HANDLING BELOW WE SHOULD HAVE
    // HERE I'M JUST LAZY. ALSO NOTE THAT SOMEONE WHO UNDERSTANDS
    // X87 NAN SHOULD TAKE A LOOK AT THIS CODE PROBABLY.
    return Status::OK.and(nan);
}
