 rust
rustc 0.10-pre (0a5138c 2014-03-04 13:16:41 -0800)


pub trait Expr<'a, T> {
    fn then(&'a self, f: &'a 'a ||) -> T;
}

impl<'a> Expr<'a, ()> for () {
    fn then(&'a self, f: &'a 'a ||) -> () {
        (*f)();
    }
}

pub struct Action<'a> {
    f: Option<&'a 'a ||>,
    next: Option<&'a Action<'a>>,
}

impl<'a> Action<'a> {
    pub fn new(f: &'a 'a ||) -> Action<'a> {
        Action { f: Some(f), next: None }
    }

    pub fn run(&self) {
        match self.next {
            Some(action) => action.run(),
            None => {},
        }

        match self.f {
            None => {},
            Some(f) => (*f)(),
        };
    }
}

impl<'a> Expr<'a, Action<'a>> for Action<'a> {
    fn then(&'a self, f: &'a 'a ||) -> Action<'a> {
        Action { f: Some(f), next: Some(self) }
    }
}

pub fn main() {
    let x = || println!("Hello!");
    let y = || println!("Again!");
    let z = || println!("So...");

    ().then(&x).then(&y).then(&z);

    Action::new(&x).then(&y).then(&z).run();
}


Compiler error:
impl2.rs:46:2: 46:4 error: cannot infer an appropriate lifetime for region in type/impl due to conflicting requirements
impl2.rs:46     ().then(&x).then(&y).then(&z);
                ^~
impl2.rs:48:2: 48:26 note: first, the lifetime must be contained by the method call at 48:1...
impl2.rs:48     Action::new(&x).then(&y).then(&z).run();
                ^~~~~~~~~~~~~~~~~~~~~~~~
impl2.rs:48:2: 48:17 note: ...so that method receiver is valid for the method call
impl2.rs:48     Action::new(&x).then(&y).then(&z).run();
                ^~~~~~~~~~~~~~~
impl2.rs:46:10: 46:12 note: but, the lifetime must also be contained by the expression at 46:9...
impl2.rs:46     ().then(&x).then(&y).then(&z);
