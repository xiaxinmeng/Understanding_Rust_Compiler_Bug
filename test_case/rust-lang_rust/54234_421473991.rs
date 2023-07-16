rust
type Alias = ();

fn fun() -> impl FnOnce() -> Alias {
    || ()
}

fn main() {
    fun()();
}
