
let c = Cell::new(vec![123]);
c.update(|vec| {
    let x = &mut vec[0];            // x references 123
    c.update(|vec| vec.clear());    // x references uninitialised memory
    *x = 456;                       // BOOM! undefined behavior
})
