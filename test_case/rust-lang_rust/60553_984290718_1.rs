rust
/* #[repr(u8)] */
enum FieldlessExplicit {
    Tuple() /* = 1 */,
    Struct{} /* = 3 */,
    Unit /* = 9 */,
}
