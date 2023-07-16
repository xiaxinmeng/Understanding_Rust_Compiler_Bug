rust
struct Struct;
struct Other;
trait Trait {}
impl Trait for Struct {}
impl Trait for Other {}
// Commented out because they cause earlier errors that stop the world, but I will cater to them later.
//fn foo() -> Foo<dyn Trait> { Struct } //~ ERROR E0277
#[allow(bare_trait_objects)]
fn fuz() -> (usize, Trait) { (42, Struct) } //~ ERROR E0277
fn bar() -> (usize, dyn Trait) { (42, Struct) } //~ ERROR E0277
//#[allow(bare_trait_objects)]
//fn baz() -> (Iterator<Item = u32>, usize) { Struct } //~ ERROR E0277
//fn bat() -> (dyn Iterator<Item = u32>, usize) { Struct } //~ ERROR E0277
#[allow(bare_trait_objects)]
fn bap() -> Trait { Struct } //~ ERROR E0277
fn ban() -> dyn Trait { Struct } //~ ERROR E0277
fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0277
fn bal() -> dyn Trait { //~ ERROR E0277
    if true {
        return Struct;
    }
    Other
}
fn main() {}
