rust
use foo::bar; //~ ERROR no module `foo` in the crate root
//  ^^^^^^^^ `foo` undeclared
//  ^^^ undeclared (pending better span info)

mod baz {}
use baz::bar; //~ ERROR no item `bar` in `baz`
//       ^^^ undeclared

fn main() {
    foo; //~ ERROR no value `foo` in scope
//  ^^^ not in scope
}
