rust
struct Data<T: ?Sized, U> where T: Object<U> {
    data: <T as Object<U>>::Output
}

trait Object<U> {
    type Output;
}

trait Mark {
    type Output;
}

impl<T: ?Sized, U: Mark<Output=V>, V> Object<U> for T {
    type Output = V;
}

fn iso_1<T, U>(data: T) -> Data<dyn Object<U, Output=T>, U> {
    // in any coherence-less solution, this shouldn't "see" the
    // blanket impl, as it might not apply (e.g., `Local` might
    // be in a third-party crate).
    Data { data }
}

fn iso_2<X: ?Sized, U, V>(data: Data<X, U>) -> V
    where U: Mark<Output=V>
{
    // similarly, this shouldn't "see" the trait-object impl.
    data.data
}

fn transmute_m<T, U, V>(data: T) -> V
    where U: Mark<Output=V>
{
    // This function *also* shouldn't see the blanket impl - `Local`
    // might be in a third-party crate.
    iso_2::<dyn Object<U, Output=T>, U, V>(
        iso_1::<T, U>(data)
    )
}

// These 3 functions could be in a child crate

struct Local<T>(T);

impl<T> Mark for Local<T> {
    type Output = T;
}

fn transmute<T, U>(x: T) -> U {
    // and this function shouldn't see *anything* that looks like a
    // problematic associated type.
    transmute_m::<_, Local<_>, _>(x)
}
