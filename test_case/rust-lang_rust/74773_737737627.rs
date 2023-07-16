
 pub fn split_n<'a, P: Pattern<'a>>(&'a self, delimiter: P, n: usize) -> Option<[&'a str; n]>;
