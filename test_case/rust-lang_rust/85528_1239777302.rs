
let it = (0..100).inspect(|x| println!("{x} "));
it.take(51).zip(0..100).next_back();
