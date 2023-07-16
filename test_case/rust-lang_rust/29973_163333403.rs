
struct Private1;
struct Private2;

pub fn f1(this: &Private1, arg: &Private2) {}

impl Private1 {
    pub fn f2(self: &Private1, arg: &Private2) {}
}
