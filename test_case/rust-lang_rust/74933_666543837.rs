rust
use std::ops::{Index, IndexMut};

pub struct T;
impl T {
    fn test(&mut self) {
    }
}

pub struct FieldMap;

pub fn insert<'a, 'b>(this: &'a mut FieldMap, field: &'b Field<'a>) -> () {
    this[field].test();
}

pub struct Field<'a> {
    x: &'a T,
}

impl<'a> Index<&'a Field<'a>> for FieldMap {
    type Output = T;

    fn index(&self, _: &'a Field<'a>) -> &T {
        unimplemented!()
    }
}

impl<'a> IndexMut<&'a Field<'a>> for FieldMap {
    fn index_mut(&mut self, _: &'a Field<'a>) -> &mut T {
        unimplemented!()
    }
}
