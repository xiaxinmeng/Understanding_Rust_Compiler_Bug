
pub trait Dimension {
   type IndexType;
}

// error: wrong number of type arguments: expected 1, found 0
// the error indicates the Dimension trait
pub trait RemoveAxis : Dimension { .. }
