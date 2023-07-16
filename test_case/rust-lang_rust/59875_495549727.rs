
use othercrate::{InvariantPtr, Ptr}

struct MyType {}

// InvariantPtr is the original code.
// This complies with no problems.
impl PartialEq<MyType> for InvariantPtr<MyType> {
    fn eq(&self, other: &MyType) -> bool {
        unimplemented!()
    }
}

// Ptr is the code with the workaround.
// This fails with:
// error[E0210]: type parameter `<MyType as othercrate::Pointee>::Meta` must be used as the type 
// parameter for some local type (e.g., `MyStruct<<MyType as othercrate::Pointee>::Meta>`)
// ... ::Pointee>::Meta` must be used as the type parameter for some local type
//  
//  = note: only traits defined in the current crate can be implemented for a type parameter
//
impl PartialEq<MyType> for Ptr<MyType> {
    fn eq(&self, other: &MyType) -> bool {
        unimplemented!()
    }
}
