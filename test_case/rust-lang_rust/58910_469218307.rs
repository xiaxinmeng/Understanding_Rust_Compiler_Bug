rust
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Field, Fields};

// Uncommenting the two lines below trigger a borrowck error.

pub fn struct_fields_mut(
    input: &mut DeriveInput,
) -> Result<impl Iterator<Item = &mut Field>, Error> {
    if let Data::Struct(ref data_struct) = input.data {
        if let Fields::Named(_) = data_struct.fields {
            // Repeat the borrow here, where it can never
            // reach the latter use
            if let Data::Struct(ref mut data_struct) = input.data {
                return Ok(data_struct.fields.iter_mut());
            } else {
                panic!()
            }
        }
    }
    Err(Error::new(
        input.span(),
        "only structs can be automatically neutralized",
    ))
}

fn main() { }
