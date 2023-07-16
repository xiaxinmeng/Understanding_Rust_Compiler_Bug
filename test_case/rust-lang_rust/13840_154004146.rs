
fn isum<'a, I: Iterator<Item=&'a i32>>(it: I) -> i32 { it.fold(0, |a, b| a + *b)}
