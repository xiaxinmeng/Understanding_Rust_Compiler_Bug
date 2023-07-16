rust
#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

trait AlsoAdd {
    fn add(self);
}

struct MyStruct(i32);

impl Add for MyStruct {
    type Output = i32;
    fn add(self, rhs: MyStruct) -> i32 {
        self.0 + rhs.0
    }
}

impl AlsoAdd for MyStruct {
    fn add(self) { }
}

fn foo() ->i32 {
    MyStruct(35) + MyStruct(42)
}
