 rust
pub enum Void {}

pub trait Display {
    // ...
    type AsStr = Void;
    fn fmt_as_str(&self) -> &<Self as Display>::AsStr { unimplemented!() }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T: Display<AsStr=Void> + ?Sized> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl<T: Display<AsStr=str> + ?Sized> ToString for T {
    fn to_string(&self) -> String {
        String::from_str(self.fmt_as_str())
    }
}
