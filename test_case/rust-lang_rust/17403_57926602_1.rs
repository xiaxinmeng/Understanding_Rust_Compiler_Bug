
test.rs:10:9: 10:15 error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements
test.rs:10         self.x
                   ^~~~~~
test.rs:9:5: 11:6 note: consider using an explicit lifetime parameter as shown: fn call(&'a self, _: ()) -> &'a mut uint
test.rs:9     fn call(&self, _: ()) -> &'a mut uint {
test.rs:10         self.x
test.rs:11     }
