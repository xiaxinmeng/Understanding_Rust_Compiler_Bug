
let it = (0..100).inspect(|x| println!("{x} "));
let it = it.take(50).zip(0..100)
it.by_ref().take(49).rfind(|_| false);
it.next()
