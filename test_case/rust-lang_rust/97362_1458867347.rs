rust
let f = for<'a> |a: &'a u8| -> impl 'a + Fn() -> &'a u8 {
    || a
};
