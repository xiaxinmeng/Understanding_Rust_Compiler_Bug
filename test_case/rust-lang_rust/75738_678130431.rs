rust
enum State {
    Start,
    Iterator,
    Separator,
}

struct Join<T: Clone, I: Iterator<Item=T>> {
    iter: I,
    item: Option<T>,
    sep: T,
    state: State,
}

impl<T: Clone, I: Iterator<Item=T>> Iterator for Join<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Start => {
                self.state = State::Separator;
                self.iter.next()
            }
            State::Iterator => {
                let item = self.item.take();
                self.state = State::Separator;
                item
            }
            State::Separator => {
                self.item = Some(self.iter.next()?);
                self.state = State::Iterator;
                Some(self.sep.clone())
            }
        }
    }
}

fn main() {
    let iter = vec![1, 2, 3].into_iter();
    let join = Join { iter, sep: 0, item: None, state: State::Start };
    for x in join {
        dbg!(x);
    }
}
