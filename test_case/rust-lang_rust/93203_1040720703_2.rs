rust
scope(|s| {
    // s here has type `&'a Scope` for some fresh lifetime 'a that is confined to this closure
    let s0 = s;
    
    s.spawn(|s| {
        // here, we shadow `s`; you get back a `&'b Scope` for some fresh `'b` lifetime specific to *this* closure
        // in reality, though, this `s` is the same as the outer one -- i.e., same pointer:
        assert_eq!(s as usize, s0 as usize);
    });
});
