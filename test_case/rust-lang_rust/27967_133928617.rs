
macro_rules! foo {
    ($key:pat) => {
        match "foobar" {
            $key => "",
            _ => "",
        }
    }
}

fn main() {
    foo! {
        concat!("foo", "bar")
    };
}
