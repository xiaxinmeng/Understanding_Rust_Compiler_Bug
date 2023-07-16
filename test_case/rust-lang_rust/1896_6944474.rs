
fn zipWith<A:copy,B:copy,C>(f: fn@((A,B))->C) -> fn(+~[A]) -> fn(+~[B]) -> ~[C] {
    fn@(+a: ~[A]) -> fn(+b: ~[B]) -> ~[C] {
        fn@(+b: ~[B], move a) -> ~[C] {
            vec::zip(a,b).map(f)
        }
    }
}

fn sum(x: (int,int)) -> int { let (y,z) = x; y+z }

fn main() {
    for zipWith(sum)(~[1,2,3,4])(~[10,20,30,40]).each |i| {
        io::println(#fmt("%d", i));
    }
}
