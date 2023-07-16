rust
fn main() {}

pub mod a {
    pub struct FontFamily;

    impl FontFamily {
        pub fn to_css(&self) -> ::std::fmt::Result {
            Ok(())
        }
    }
}

macro_rules! reexport {
    () => {
        mod b {
            pub use a;
        }
    }
}

reexport!();

pub mod c {
    use b::a::FontFamily;

    struct FontFace {
        family: FontFamily
    }

    impl FontFace {
        pub fn to_css(&self) -> ::std::fmt::Result {
            try!(self.family.to_css());
            Ok(())
        }
    }
}