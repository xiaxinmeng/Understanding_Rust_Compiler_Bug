rust
trait Lt<'a> {
    type T;
}
struct Id<T>(T);
impl<'a,T> Lt<'a> for Id<T> {
    type T = T;
}

fn main() {
    let _v: <Id<()> as Lt<'_>>::T = ();
    let _f: fn(_) = |_:<Id<()> as Lt<'_>>::T| {};
    //ICE: broken MIR in DefId
    //_f(_v)
}
