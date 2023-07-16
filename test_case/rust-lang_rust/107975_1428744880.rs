rs
// Produces true on following println

black_box(format_args!("{:?}", a));
//black_box(format_args!("{:?}", b));
//format!("{:?}", a);

// Doesn't

//format_args!("{:?}", black_box(a));
//black_box(format_args!("{:?}", black_box(a)));
//format!("{:?}", black_box(a));
