rust
enum Empty1 {}

enum Empty2 {}

fn foo(x: (Empty1, Empty2)) {
    match x {} // ERROR: missing match arm.
}
