

impl<'a, A: PartialOrd + Copy, B> Tree<'a, A ,B> {
    fn deep_fetch(&mut self, value: Either<A, B>) -> Result<&mut Self, (&mut Self, Either<A,B>)> {
        match (self, value) {
            (Tree::ABranch(ref mut a, ref v), Either::Left(ref vv)) if v == vv => 
                a.deep_fetch(Either::Left(*vv)),

            (this, _v) => Err((this, _v))
        }
    }
}
