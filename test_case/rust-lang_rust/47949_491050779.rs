rust
fn foo() -> NoisyDrop {
    let n = NoisyDrop::new(1);
    let x = {
        let p = PanickyDrop;
        NoisyDrop::new(2) // <-- dtor for this never runs 
    };
    panic!()
}
