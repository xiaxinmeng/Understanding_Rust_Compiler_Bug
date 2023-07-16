
/// Returns a Iterator over the graphemes of a string.
///
/// Which string iterator do I need?
/// - "aỹe".iter_graph() => iterates "a", "ỹ", "e"
/// - "aỹe".iter_cp()    => iterates 'a', 'y', '\u0303', 'e'
/// - "aỹe".iter_bytes() => iterates 0x61, 0x79, 0xcc, 0x83, 0x65
fn iter_graph() ...
