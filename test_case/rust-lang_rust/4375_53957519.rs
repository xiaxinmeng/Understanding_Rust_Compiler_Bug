 rust
macro_rules! do_bar(
    () => ({ // extra { here.
        let x = 1u;
        if x == 1 {return}
    })
)
