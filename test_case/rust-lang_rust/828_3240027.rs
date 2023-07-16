
mod Foo {
     type t = uint;
     fn foo() -> t { ret 32u; }
}

mod Bar {
     type t = int;
     fn bar() {
         let x: t = Foo::foo(); // Error: uint vs int mismatch
     }
}

fn main() {
}
