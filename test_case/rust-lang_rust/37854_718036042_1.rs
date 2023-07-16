rust
match x {
    0..9 => foo(),
    9.. => bar(), // Would fail to compile if you tried `10..` due to exhaustiveness checking
}
