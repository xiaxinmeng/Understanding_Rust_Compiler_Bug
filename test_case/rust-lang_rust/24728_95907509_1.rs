 rust
fn main() {
    trait Trait { type AssociatedType; }
    impl Trait for i8 { type AssociatedType = char; }
    fn foo<T>(t: T) where T: Trait<AssociatedType=u32> {
        println!("in foo");
    }

    foo(3i8);
}
