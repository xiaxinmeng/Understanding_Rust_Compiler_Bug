rust
#![deny(no_mangle_generic_items)]
trait Trait {
    #[no_mangle] fn foo() {} //~ ERROR functions generic over types or consts must be mangled
}
