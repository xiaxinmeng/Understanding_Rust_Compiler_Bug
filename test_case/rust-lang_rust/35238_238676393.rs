 rust
macro_rules! MACRO {
    () => {
        bbb();
        let ccc = 1; 
        ddd();
    }
}

fn function() {
    aaa();
    MACRO!();
    eee();
}
