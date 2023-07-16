
trait FromVec<'a> {
    fn fromVec(&'a [u8]) -> Self;
}

fn get<'a, T : FromVec<'a>>(v : &'a [u8]) -> T {
    FromVec::fromVec(v)
}

fn main() {}
