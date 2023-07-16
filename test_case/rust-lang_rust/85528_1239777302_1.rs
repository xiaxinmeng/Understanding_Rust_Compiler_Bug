
let it = (0..100).inspect(|x| println!("{x} "));
it.take(50).zip(0..100).rfind(|_| false);
