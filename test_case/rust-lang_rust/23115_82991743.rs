 rust
// binary operators
$e1 + $e2

// unary operators
! $e

// closures
|x| $e

// parameters
fn foo($e: $t) {}

// type parameters
fn bar<$ident: $type>

// type ascription
$e : $type

// path expressions (?)
$x::foo()

// C-style enums
enum Foo {
    $ident = $e,
}

// derived types
& &mut $t

// call expressions (?)
$e()

// ... and who knows what else ...
