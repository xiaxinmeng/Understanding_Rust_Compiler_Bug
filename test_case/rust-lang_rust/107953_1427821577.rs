rust
macro_rules! static_unreachable {
    () => {
        #[allow(unreachable_code)]
        {
            let s = String::new();
            let _a = s;
            let _b = s;
        }
    };
}
