rust
// Extracts the value type of an arbitrarily nested tuple
trait NestedTupleType<T> {}

// Identity case: Every T is the innermost type of a nested tuple of type T.

// Examples:
// i32 is a 0-deep nested tuple of value type i32
// (i32,) is a 0-deep nested tuple of value type (i32,)
// ((i32,),) is a 0-deep nested tuple of value type ((i32,),)
// (((i32,),),) is a 0-deep nested tuple of value type (((i32,),),)
// etc.

impl<T> NestedTupleType<T> for T {}

// Nested case: Every (U,) is a >=1D array of type T.
// In other words, (U,) must be one of (T,); ((T,),); (((T,),),); etc.

// Examples:
//
// (i32,) is a 1-deep nested tuple of value type i32
//
// ((i32,),) is 1-deep nested tuple of value type (i32,)
// ((i32,),) is also a 2-deep nested tuple of value type i32
//
// (((i32,),),) is a 1-deep nested tuple of value type ((i32,),)
// (((i32,),),) is also a 2-deep nested tuple of value type (i32,)
// (((i32,),),) is *also* a 3-deep nested tuple of value type i32
//
// etc.

impl<T, U: NestedTupleType<T>> NestedTupleType<T> for (U,) {}
