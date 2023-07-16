
fn something<T>(e: T) { ... }

// This works
let tup = (something::<int>, 42i);
tup.0();

// This doesn't
let tup = (something, 42i);
           ^ unable to infer enough type information about `_`; type annotations required
tup.0()::<int>;
       ^ found `::` expecting `;` or `}`
