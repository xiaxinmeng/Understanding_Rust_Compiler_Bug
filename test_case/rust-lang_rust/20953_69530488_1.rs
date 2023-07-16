 text
i.rs:3:19: 3:22 error: the trait `Trait` is not implemented for the type `&Trait`
i.rs:3 fn a(x: &Trait) { try(x); }
                         ^~~
i.rs:4:23: 4:26 error: the trait `Trait` is not implemented for the type `&mut Trait`
i.rs:4 fn b(x: &mut Trait) { try(x); }
                             ^~~
i.rs:5:23: 5:26 error: the trait `Trait` is not implemented for the type `Box<Trait>`
i.rs:5 fn c(x: Box<Trait>) { try(x); }
                             ^~~
error: aborting due to 3 previous errors
