 rust
enum Type<T> { Constant }

trait Trait<K,V> {
    fn method(&self,Type<(K,V)>);
}

impl<V:Clone+'static> Trait<u8,V> for () {
    fn method(&self, _: Type<(u8,V)>) {}
}

fn main () {
    let a = @() as @Trait<u8, u8>;
    a.method(Constant);
}
