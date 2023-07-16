rust

#![feature(nll)]
#![nll]
#![allow(dead_code)]

//#[derive(Debug)] //no need for this example
enum Opt<T> {
    //a non-Copy Option<T> XXX actually Option<T> is Copy only when T is Copy ; not so for this Opt<T> which is never Copy
    Some(T),
    None,
}

impl<T> Opt<T> {
    #[inline]
    pub fn as_mut(&mut self) -> Opt<&mut T> {
        match *self {
            Opt::Some(ref mut x) => Opt::Some(x),
            Opt::None => Opt::None,
        }
    }
    #[inline]

    pub fn unwrap(self) -> T {
        match self {
            Opt::Some(val) => val,
            Opt::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

struct WrappedI32(i32); //avoid Copy
struct Foo(Opt<WrappedI32>);

impl Foo { //example stolen&modified from jmcomets from irc
    fn get_or_set(&mut self, value: WrappedI32) -> &mut WrappedI32 {
        /*match self.0 {//this compiles, but not entirely sure it's equivalent to 'case 4' below
            Opt::Some(ref mut value) => return value,
            Opt::None => {
                self.0 = Opt::Some(value);
                self.0.as_mut().unwrap()
            }
        }*/
        {//this doesn't compile: that is, cases 3&4 don't work
            
            
            //case 1: compiles
            /*
            let a = self.0.as_mut();
            return a.unwrap();
            */
            
            //case 2: compiles
            /*
            let a = self.0.as_mut();
            */
            
            //case 3: fails to compile
            // /*
            let a = self.0.as_mut();
            if 1 == 2 {
                return a.unwrap();
            }
            // */
            
            //case 4: fails to compile
            /*
            if let Opt::Some(value) = self.0.as_mut() { //allowing this block compile fails
                return value;
            }
            */
        }
        self.0 = Opt::Some(value); //compile fails is here: error[E0506]: cannot assign to `self.0` because it is borrowed
        self.0.as_mut().unwrap()
    }
}

fn main() {
    // ...
}
