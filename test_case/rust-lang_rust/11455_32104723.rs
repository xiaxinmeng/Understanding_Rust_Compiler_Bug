
fn evil<T, Iter: Iterator<T>>(iter: Iter) -> ~Iterator:<T> {
    ~iter as ~Iterator:<T>
}
