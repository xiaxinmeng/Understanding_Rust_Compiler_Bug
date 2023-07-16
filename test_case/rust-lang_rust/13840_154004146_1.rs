
fn isum<I: Iterator<Item=&i32>>(it: I) -> i32 { it.fold(0, |a, b| a + *b)}
