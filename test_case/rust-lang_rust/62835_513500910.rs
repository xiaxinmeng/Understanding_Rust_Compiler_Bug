
let place = MaybeUninit::<Vec<()>>::uninit();
place.write(vec![]);
// implicit Copy
let vec_1 = unsafe { place.assume_init() };
// Boom.
let vec_2 = unsafe { place.assume_init() };
