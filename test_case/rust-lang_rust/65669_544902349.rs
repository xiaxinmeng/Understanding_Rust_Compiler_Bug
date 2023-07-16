rust
let odds = iter.filter((|&a| a % 2 == 0).negate()) // which is much less readable than
let odds = iter.filter(|&a| a % 2 != 0) // and
let odds = iter.filter(|&a| !(a % 2 == 0))
