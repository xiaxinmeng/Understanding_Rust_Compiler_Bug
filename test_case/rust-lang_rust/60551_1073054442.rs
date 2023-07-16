Rust
#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait MathTrait {

    const NDIM: usize;
    fn calculate_math(&mut self, data: &mut [f64]);
    
}

struct Solver<'a, M>
where
    M: MathTrait + Sized,
    [f64; M::NDIM]: Sized,
{
    pub x: [f64; M::NDIM],
    problem: &'a mut M,
}

impl<'a, M> Solver<'a, M> 
where
    M: MathTrait + Sized,
    [f64; M::NDIM]: Sized,
{
    pub fn new(problem: &'a mut M ) -> Solver<'a, M> {
        Solver::<'a, M> {
            x: [0.0_f64; M::NDIM],
            problem,
        }
    }
}

trait PL {
    const NSIZE: usize;
}

struct PhysicsComp{}

impl PL for PhysicsComp {
    const NSIZE: usize = 32;
}

struct Physics<P: PL> {
    comp: P,
}

impl<P: PL> MathTrait for Physics<P> 
where
    [f64; P::NSIZE]: Sized,
{
    const NDIM: usize = P::NSIZE;
    
    fn calculate_math(&mut self, data: &mut [f64]) {
        /* do math here or whatever */
    }
}

struct DoStuff<P: PL> {
    data: P,
}

impl<P: PL> DoStuff<P>
where
    [f64; P::NSIZE]: Sized, // Bound that is equivalent to the error of [f64; M::NDIM]: Sized,
{
    fn problem(&self) -> bool {
        let mut phys = Physics {
            comp: PhysicsComp {},  
        };
        // This line fails quite hard
        let mut solver = Solver::<Physics::<P>>::new(&mut phys);
        true
    }
}


fn main() {
    let failure = DoStuff {
        data: PhysicsComp {},
    };
    failure.problem();
}
