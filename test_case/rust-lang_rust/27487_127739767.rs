rust
/// Identity function. Move the input to output.
fn moving<T>(x: T) -> T { x }

let current: &mut _;

// Force move like this
moving(current).next

// we can also write it like this:
let tmp = current;
tmp.next

// or this:
{current}.next
