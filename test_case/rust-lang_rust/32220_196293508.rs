 rust
//This is a regular trait to enforce a single implementation per type
trait Cons {
    type Head;
    type Tail;
}

marker Contains<T>;

impl<N, T, Hstck: Cons<Head=N, Tail=T>> Contains<N> for Hstck;
impl<N, H, T, Hstck: Cons<Head=H, Tail=T>> Contains<N> for Hstck where T: Contains<N>;
