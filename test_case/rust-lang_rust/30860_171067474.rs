 rust
trait TypeWithLifetime<'a> {
    type Type;
}

type At<'a,T> where T: TypeWithLifetime<'a> = T::Type;

fn main() {
    let _ = |x:At<()>| false;
}
