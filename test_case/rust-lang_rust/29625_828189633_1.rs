
struct A { }
impl A {
    pub fn my_mut(&mut self) { self.my_once() }
    pub fn my_once(self) { }
}
