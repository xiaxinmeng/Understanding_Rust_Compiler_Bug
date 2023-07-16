rust
union OneOrManyBytes {
    one: u8,
    many: [u8]
}
size_of_val(&OneOrManyBytes { one: 0 })
