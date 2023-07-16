rust
#![feature(associated_type_bounds)]

trait Load : Sized {
    type Blob : Sized;
}

trait Primitive { } 

impl<T: Primitive> Load for T { 
    type Blob = Self;
}

trait BlobPtr : Primitive { } 

trait CleanPtr : Load<Blob : BlobPtr> {
    fn to_blob(&self) -> Self::Blob;
}


impl Primitive for () { } 
impl BlobPtr for () { } 

fn main() {
}
