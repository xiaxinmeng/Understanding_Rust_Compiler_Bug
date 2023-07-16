rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

struct One<'a>(&'a mut [u8]);

impl<'a> One<'a> {
    fn step(self) -> Either<Two<'a>, OneSave> { unimplemented!() }
}

impl<'a> Drop for One<'a> {
    fn drop(&mut self) {}
}

struct OneSave;

impl OneSave {
    fn resume(self, _: &mut [u8]) -> One<'_> { unimplemented!() }
}

struct Two<'a>(&'a mut [u8]);

impl<'a> Two<'a> {
    fn step(self) -> One<'a> { unimplemented!() }
}

fn grow(_: &mut Vec<u8>) -> &mut [u8] { unimplemented!() }

pub fn go(stuff: &mut Vec<u8>) {
    let mut one = One(stuff);
    loop {
        let two = loop {
            one = match one.step() {
                Either::Left(x) => break x,
                Either::Right(save) => save.resume(grow(stuff)),
            };
        };
        one = two.step();
    }
}
