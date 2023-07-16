rs
// Input seperate repetitions
$($first:ident).*: $($second:ident + $($third:ident)*).*
// Output shared
$($first:ident = $(third:ident)*)*
