rust
let place = MaybeUninit::<Vec<()>>::uninit();

place.write(vec![]);

// because they're calling "clone", user might think this is safe;
// but in reality, the same vec is read twice.

let vec_1 = unsafe { place.clone().assume_init() };
let vec_2 = unsafe { place.clone().assume_init() };
