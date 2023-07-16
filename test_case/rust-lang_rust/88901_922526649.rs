rust
// FIXME(matthewjasper) This allows copying a type that doesn't implement
// `Copy` because of unsatisfied lifetime bounds (copying `A<'_>` when only
// `A<'static>: Copy` and `A<'_>: Clone`).
// We have this attribute here for now only because there are quite a few
// existing specializations on `Copy` that already exist in the standard
// library, and there's no way to safely have this behavior right now.
#[rustc_unsafe_specialization_marker]
