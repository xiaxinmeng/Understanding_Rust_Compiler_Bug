 rust
trait TestTrait {

}

fn foobar<'a, B1 = &'a i32>(_n:&i32)
    where B1: TestTrait + 'a
{
}

fn main() {
    foobar(&52);
}
