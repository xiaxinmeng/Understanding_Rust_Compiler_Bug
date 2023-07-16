
/// Invokes `try_fn(v)` and then `finally_fn(v)`. Always invokes
/// `finally_fn(v)`, even if `try_fn(v)` fails.
fn try_finally<A>(v: &mut A, try_fn: |&mut A|, finally_fn: |&mut A|) {...}
