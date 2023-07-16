 rust
trait Trait_A {}
struct Struct_A;
impl Trait_A for Struct_A {}


trait Trait_B<A> {}
struct Struct_B<A>;
impl<A:Trait_A> Trait_B<A> for Struct_B<A> {}

struct C<A,B>;

trait Trait_D<'a, A:Trait_A, B:Trait_B<A>> {
    fn c(&self) -> &'a C<A,B>;
}

struct Struct_D<'a,A,B> {
    c: &'a C<A,B>,
}

impl<'a,A:Trait_A,B:Trait_B<A>> Trait_D<'a,A,B> for Struct_D<'a,A,B> {
    fn c(&self) -> &'a C<A,B> {
        self.c
    }
}

fn foo<'a, A: Trait_A, B: Trait_B<A>, D: Trait_D<'a, A, B>>(d: &'a D) -> uint {
    0u
}

pub fn main() {
    let a = Struct_A;
    let b: Struct_B<Struct_A> = Struct_B;
    let c: C<Struct_A, Struct_B<Struct_A>> = C;
    let d = Struct_D { c: &c };

    let z = foo(&d); 

    println!("{}", z.to_str());
}
