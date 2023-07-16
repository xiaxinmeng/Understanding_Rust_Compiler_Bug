rust
pub fn insert<'a, 'b>(this: &'a mut FieldMap, field: &'b Field<'a>, value: ()) -> () {
    this[field] = value;
}
