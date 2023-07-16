 rust
struct A(uint);

impl A {
     fn b(self: &A(ref mut u), i: uint) { //~ error: expected identifier, found keyword `ref`
//~^ error: expected one of `(`, `+`, `,`, `::`, or `<`, found `mut`
        *u = i;
    }
}
