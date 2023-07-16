
macro_rules! make_accessors {
    (
            struct $struct_name:ident {
                    $(
                            $(#[$field_meta:meta])*
                            $field_vis:vis $field_name:ident : $field_ty:ty
                            $(: get $getter_ty:ty)?,
                    )* $(,)?
            }
    ) => {
            struct $struct_name {
                    $(
                            $(#[$field_meta])*
                            $(
                                    $field_name : $getter_ty, // put it there
                            )?
                    )* // remove the comma here
            }
    }}
    make_accessors! {
            struct A {
                    a: u32 : get u32,
                    b: u32,
                    c: u32 : get u32,
            }
    }
    
    fn main() {
    }
