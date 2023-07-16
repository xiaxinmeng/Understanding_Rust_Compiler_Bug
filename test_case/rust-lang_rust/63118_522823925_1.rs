rust
let return_place = match scrutinee {
    binding if fun(&binding) => binding,
    ...
}
