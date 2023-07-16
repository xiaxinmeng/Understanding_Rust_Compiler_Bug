
trait CLikeEnum  : FromPrimitive, ToPrimitive {
    fn last() -> Self; // Returns the "last" enum, traditionally the one with maximum value
}
