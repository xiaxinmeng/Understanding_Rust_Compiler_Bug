rust
  trait Foo {
    type A;
}

struct StructA;
struct StructB;

impl Foo for StructA {
    type A = ();
}

// Warning about not enforced trait bounds
type B<A: Foo> = A::A;

// Error with the message:  associated type `A` not found for `A`
// -> Trait bound is required
//type C<A> = A::A

type D = B<StructA>;
// As the warning says this works
type E = B<StructB>;
