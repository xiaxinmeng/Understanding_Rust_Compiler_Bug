 rust
struct Inline {
    x: Unsafe<uint>
}

struct Pointer<'a> {
    y: &'a Unsafe<uint>
}

let inline: Inline = ...;
let ptr: Pointer = ...;

foo(&inline); // warn
foo(inline); // no warning: there's no way for `inline` itself to be visibly mutated

foo(&ptr); // warn
foo(ptr); // warn
