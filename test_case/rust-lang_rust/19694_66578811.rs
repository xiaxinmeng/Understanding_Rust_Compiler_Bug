 rust
// We don't want to recurse into anything other than mods, since
// mods or tests inside of functions will break things
let res = match i.node {
    ast::ItemMod(..) => fold::noop_fold_item(i, self),
    _ => SmallVector::one(i),
};
