
error[E0369]: binary operation `==` cannot be applied to type `&Constructor<'_>`
   --> compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:821:55
    |
821 |             Variant(_) => used_ctors.iter().any(|c| c == self),
    |                                                     - ^^ ---- &Constructor<'_>
    |                                                     |
    |                                                     &Constructor<'_>
    |
    = note: an implementation of `std::cmp::PartialEq` might be missing for `&Constructor<'_>`
