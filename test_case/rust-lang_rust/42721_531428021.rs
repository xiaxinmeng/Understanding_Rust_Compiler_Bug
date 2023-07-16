rust
#![feature(specialization)]

trait Integer {
    fn to_int(&self) -> i32;
}

trait Collection {
    fn to_vec(&self) -> Vec<i32>;
}

trait PrintAnything {
    fn print(&self) -> String;
}

impl<T> PrintAnything for T {
    default fn print(&self) -> String {
        format!("unprintable")
    }
}

impl<T: Integer> PrintAnything for T {
    fn print(&self) -> String {
        format!("int {}", self.to_int())
    }
}

impl<T: Collection> PrintAnything for T {
    fn print(&self) -> String {
        format!("collection {:?}", self.to_vec())
    }
}
