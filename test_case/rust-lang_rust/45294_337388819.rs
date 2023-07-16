
IMPL_TRAIT = `impl` BOUND
    | `impl` `(` BOUNDS `)`
BOUNDS = Ø | BOUND (`+` BOUND)* `+`? 
