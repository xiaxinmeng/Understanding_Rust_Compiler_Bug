rust
let mut it = /* `zip` of TrustedRandomAccess iterators */;

it.next(); // only increases `index` field of `Zip`
it.try_fold( ... ); // this PR: starts iteration from the begenning
