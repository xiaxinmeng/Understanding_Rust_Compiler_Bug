 rust
trait Seq<T>: Default {
    fn add_elem(&mut self, x: T);
}

macro_rules! seq {
    ($($e: expr),*) => {{
        let mut _thing = Default::default();
        $( _thing.add_elem($e); )*
        _thing
    }}
}

let s: HashSet<int> = seq!(1, 2, 3); // HashSet impls Seq<int>
let s: HashMap<int, int> = seq!((1, 10), (2, 20), (3, 30); // impls Seq<(int, int)>
