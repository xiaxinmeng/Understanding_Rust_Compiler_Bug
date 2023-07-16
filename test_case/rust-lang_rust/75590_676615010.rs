rust
// currently requires both to be ShallowEq (or not), but one could imagine a full matrix of 4 options
impl<P: Pointer + PartialEq, T: Tag + PartialEq> PartialEq for TaggedPtr<P, T> {}
impl<P: Pointer + ShallowEq, T: Tag + ShallowEq> PartialEq for TaggedPtr<P, T> {}

impl<P: Pointer + Eq, T: Tag + Eq> Eq for TaggedPtr<P, T> {}
impl<P: Pointer + ShallowEq, T: Tag + ShallowEq> Eq for TaggedPtr<P, T> {}
