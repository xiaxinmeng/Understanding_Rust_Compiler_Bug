rust
struct Cell<Fg, Bg = Fg> {
    foreground: Color<Fg>,
    background: Color<Bg>,
}

trait Over<Bottom, Output> {
    fn over(self) -> Output;
}

impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    Self: Over<Color<BottomBg>, Cell<NewFg>>,
{
    fn over(self) -> Cell<NewFg> {
        self.over();
    }
}

impl<'b, TopFg, TopBg, BottomFg, BottomBg> Over<&Cell<BottomFg, BottomBg>, ()>
    for Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Over<Cell<BottomFg>, Cell<BottomFg>>,
{
    fn over(self) -> Cell<NewBg> {
        self.over();
    }
}
