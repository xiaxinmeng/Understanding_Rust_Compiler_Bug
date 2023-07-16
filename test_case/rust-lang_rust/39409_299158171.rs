rust
'a: loop {
    loop {
        if second_iter {
            break 'a; // want to generate `EndRegion('a)` here
        } else {
            &'a x
        }
    }
}
