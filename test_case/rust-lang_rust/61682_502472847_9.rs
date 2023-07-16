rust
type Alias<T> = Option<T>;

mod foo {
    pub use Alias::Some;
}

fn bar() {
    foo::Some::<u8>;
}
