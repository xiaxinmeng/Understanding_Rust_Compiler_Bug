rust
trait Try: FromBreak<Self::Break> {
    type Continue;
    type Break;
    
    fn from_continue(c: Self::Continue) -> Self;
    fn branch(self) -> ControlFlow<Self::Break, Self::Continue>;
}
