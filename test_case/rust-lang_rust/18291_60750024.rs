 rust
trait BoxedSlice_<T> for Sized? {
    //fn into_vec_(Box<self>) -> Vec<T>;  //~ error: expected identifier, found keyword `self`
    fn into_vec_(Box<Self>) -> Vec<T>;
}

impl<T> BoxedSlice_<T> for [T] {
    fn into_vec_(slice: Box<[T]>) -> Vec<T> {
        unimplemented!();
    }
}

fn main() {
    let v: Box<[u8]> = box [0u8, 1, 2];
    println!("{}", v.into_vec_());
    //~^ error: type `Box<[u8]>` does not implement any method in scope named `into_vec_`
}
