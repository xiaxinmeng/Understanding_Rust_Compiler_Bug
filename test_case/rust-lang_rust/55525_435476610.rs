rust
enum MatcherPosHandle<'a> {
    Ref(&'a mut MatcherPos<'a>),
    Box(Box<MatcherPos<'a>>),
}
