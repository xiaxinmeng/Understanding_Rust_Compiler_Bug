rust
pub trait Tr {
    fn f() // <-- new line is needed to trigger the ICE
        where Self: Sized;
}
