 rust
pub struct Referrable {
    x: i64
}

impl Referrable {
    pub fn new() -> Referrable {
        Referrable { x: 0i64 }
    }

    pub fn update(&mut self) {
        self.x += 1;
    }
}

pub trait Segger<'a> {
    fn new(&'a Referrable) -> Self;
    fn update(& self);
}

pub struct SegFault<'a> {
    r: &'a Referrable
}

impl<'a> Segger<'a> for SegFault<'a> {
    fn new(r: &'a Referrable) -> SegFault<'a> {
        SegFault { r: r }
    }

    fn update(& self) {
        println!("{}", self.r.x);
    }
}


pub struct SegObscure<'a> {
    seg: Box<Segger<'a> + 'a>
}

impl<'a> SegObscure<'a> {
    pub fn new(r: & Referrable) -> SegObscure<'a> {
        let seg: Box<SegFault> = box Segger::new(r);
        SegObscure { seg: seg }
    }

    pub fn update(& self) {
        self.seg.update();
    }
}

fn main() {
    let mut r = Referrable::new();
    let s = SegObscure::new(& r);
    for _ in range(0i, 10i) {
        s.update();
        r.update();
    }
    r = Referrable::new();
    s.update();
    // Still works!
}
