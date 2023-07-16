rust
trait Color {
    type Channel;
}

trait CloneColor: Color
where
    Self::Channel: Clone,
{}

fn foo<T: CloneColor>() {
    let f = T::Channel::clone;
}
