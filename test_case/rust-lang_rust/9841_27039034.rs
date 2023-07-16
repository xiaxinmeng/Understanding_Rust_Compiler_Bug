
fn fromVecBare<'a, T : FromVec<'a>>(v : &'a [u8]) -> T {
   FromVec::fromVec::<for T>(v)
}
