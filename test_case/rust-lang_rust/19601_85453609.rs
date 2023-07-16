 rust
trait A<T> {
    fn foo(&self) -> T;
}

struct B<T> where B<T>: A<B<T>> { t: T }
