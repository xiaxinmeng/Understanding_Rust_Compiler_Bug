
#![allow(unused)]

trait Empty {}

struct Dep<'a> (&'a mut u8);

impl Empty for Dep<'_>{}

struct A {
    deps: Vec<*const dyn Empty>,
}

impl A {
    pub fn inject(&mut self, dep: *const dyn Empty) {
        self.deps.push(dep);
    }
}

fn test(dep: &Dep<'_>) {
    let mut a = A{deps: vec![]};
    a.inject(dep);
}
