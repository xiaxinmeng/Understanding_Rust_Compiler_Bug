rust
enum Enum {
    Variant,
}

trait Proto {
    type Assoc;
}

struct Unit;

impl Proto for Unit {
    type Assoc = Enum;
}

fn foo() {
    let x = < <Unit as Proto>::Assoc >::Variant;
}
