 rust
macro_rules! defn {
    (pub $n:ident) => (
        pub fn $n (&self) -> i32 { 1 } )
}

#[derive(Copy)]
pub struct S;

impl S {
    defn!(pub f);
}


#[cfg(test)]
mod tests {
    use super::S;

    #[test]
    fn test_f() {
        let s=S;
        assert_eq!(1, s.f());
    }
}
