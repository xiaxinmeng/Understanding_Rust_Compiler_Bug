 rust
// Struct that has all the comparison operators
// Fails if anything is a float
#[deriving(Eq, Ord)]
struct Foo { ... }

// Manual implementation of the == operator (you get != for free)
impl Eq for Foo {
    fn eq(&self, other: &Foo) -> bool { ... }
}

// Manual implementation of all comparison operators
impl Ord for Foo {
    fn cmp(&self, other: &Foo) -> Ordering { ... }
}

// This function can use the == operator (same for Ord and comparison operators)
fn foo<T: Eq>() {}

// This function can also use the == operator (same for PartialOrd and comparison operators)
fn foo<T: PartialEq>() {}
