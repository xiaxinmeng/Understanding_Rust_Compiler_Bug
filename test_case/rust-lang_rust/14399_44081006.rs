 rust
fn get_property(&self, _: &str) -> PropertyType { Struct(self.v.clone() as Box<Property>) }
