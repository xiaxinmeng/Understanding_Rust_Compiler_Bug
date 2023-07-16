rust
let (a_scalar, b_scalar) = match dest.layout.abi {
    layout::Abi::ScalarPair(ref a, ref b) => (a, b),
    _ => bug!("store_with_flags: invalid ScalarPair layout: {:#?}", layout)
};
// This has been copy-pasted all over the place, I wonder how to deduplicate it
let b_offset = a_scalar.value.size(bx).align_to(b_scalar.value.align(bx).abi);

let a_align = dest.align;
let b_align = dest.align.restrict_for_offset(b_offset);
