 rust
#![feature(macro_rules)]

static BLAHBLAH: bool = true;
macro_rules! bar(
    () => ({
        BLAHBLAH
    })
)

#[cfg(test)]
mod test {
    static BLAHBLAH: bool = false;

    #[test]
    fn test() {
        assert!(bar!());
    }
}
