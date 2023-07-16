rust
macro_rules! repro {
    () => {
        {
            // does not create a lint; the outer block "uses" it, I suspect
            #[must_use] {
                let [some, complicated, expansion] = [(); 3];
                let _ = [some, complicated, expansion];
                5
            }
        }
    };
}


pub fn f() {
    repro!();
}
