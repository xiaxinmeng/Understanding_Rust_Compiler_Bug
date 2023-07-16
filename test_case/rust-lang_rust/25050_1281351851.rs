rust
#![feature(associated_type_defaults)]

pub trait NBTTypeHelper where Self: NBTType {
	type Tagtype = Self;
}

pub trait NBTType {
}

impl NBTTypeHelper for dyn NBTType {
	type Tagtype = <Self as NBTTypeHelper>::Tagtype;
}

fn main() {}
