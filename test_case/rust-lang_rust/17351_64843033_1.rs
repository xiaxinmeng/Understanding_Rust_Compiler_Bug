
ice0.rs:2:26: 2:31 error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
ice0.rs:2     let _: &[&Clone] = &[&"hi"];
                                   ^~~~~
note: cannot call a method (`clone`) whose type contains a self-type (`<generic #0>`) through a trait object
note: cannot call a method (`clone_from`) whose type contains a self-type (`&<generic #0>`) through a trait object
error: aborting due to previous error
