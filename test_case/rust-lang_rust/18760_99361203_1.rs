
:8:5: 10:6 error: method `deref` has an incompatible type for trait: values differ in mutability [E0053]
:8     fn deref<'a>(&'a mut self) -> &'a u8 {
:9         &self.0
:10     }
error: aborting due to previous error
