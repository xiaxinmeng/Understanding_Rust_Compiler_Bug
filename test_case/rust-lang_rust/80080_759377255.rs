rust
fn main() {
    let <WithStructStruct as Trait>::AssociatedType { a } = <WithStructStruct as Trait>::AssociatedType { a: 0 };
    let <WithTupleStruct as Trait>::AssociatedType(a) = <WithTupleStruct as Trait>::AssociatedType(0);
                                                     // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ This is the only thing that currently parses 
    let s = StructStruct { a : 0 };
    match s {
       <WithStructStruct as Trait>::AssociatedType { a } => a,
    };

    let s = TupleStruct(0);
    match s {
       <WithTupleStruct as Trait>::AssociatedType(a) => a,
    };
}

struct StructStruct {
    a: usize
}

struct TupleStruct(usize);

trait Trait {
    type AssociatedType;
}

struct WithStructStruct;

impl Trait for WithStructStruct {
    type AssociatedType = StructStruct;
}

struct WithTupleStruct;

impl Trait for WithTupleStruct {
    type AssociatedType = TupleStruct;
}
