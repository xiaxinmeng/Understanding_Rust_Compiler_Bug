
trait ToStrRadix { pure fn to_str_radix (radix: uint) -> str; }
impl int: ToStrRadix {
    pure fn to_str_radix (radix: uint) -> str { int::to_str(self, radix) }
}
// do the same for other types that support radix
