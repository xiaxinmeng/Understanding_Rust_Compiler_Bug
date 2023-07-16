rust
macro_rules! aa {
    () => {
        fn gg() {
            #[cfg(crossbeam_loom)]
            {
                let _ = val;
                unimplemented!("loom does not support non-atomic atomic ops");
            }
        }
    };
}

fn main() {
    aa!();
}
