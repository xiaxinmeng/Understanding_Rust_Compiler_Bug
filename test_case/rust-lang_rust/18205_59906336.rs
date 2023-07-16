 rust
trait Segger<'a> {
    fn new(&'a int) -> Self;
    fn get(&self) -> int;
}

struct SegFault<'a> {
    r: &'a int,
}

impl<'a> Segger<'a> for SegFault<'a> {
    fn new(r: &'a int) -> SegFault<'a> { SegFault { r: r } }
    fn get(&self) -> int { *self.r }
}

struct SegObscure<'a> {
    seg: Box<Segger<'a> + 'a>,
}

impl<'a> SegObscure<'a> {
    fn new(r: &'a int) -> SegObscure<'a> {
        //     ^
        // Interestingly, this lifetime can be removed with no effect
        let seg: Box<SegFault> = box Segger::new(r);
        SegObscure { seg: seg }
    }
}

fn main() {
    let mut r = vec![42i];
    let s = SegObscure::new(&r[0]);
    println!("{}", s.seg.get());

    // We can do all sorts of evil things
    r.pop();
    drop(r);

    // We can now (rather unreliably) overwrite what was in r[0] previously by
    // allocating something somewhere in memory
    let _ = box 36i;

    // ...and we've changed the value magically
    println!("{}", s.seg.get());
}
