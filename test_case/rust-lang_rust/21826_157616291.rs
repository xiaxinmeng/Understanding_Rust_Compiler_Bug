 rust
mod foo {
    pub trait Write {
        fn write(&self) {}
    }

    impl Write for u8 {}
}
mod bar {
    pub trait Write {
        fn write(&self) {}
    }

    impl Write for u8 {}
}
macro_rules! write {
    ($x: expr) => { {
        use foo::Write;
        use bar::Write;

        $x.write()
    }}
}

fn main() {
    write!(1u8);
}
