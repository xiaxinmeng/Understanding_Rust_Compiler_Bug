rust
pub struct S;

fn check() -> bool {
    true
}

impl S {
    fn impl_get_slice(&self, _val: &()) -> &[()] {
        &[]
    }

    #[inline(always)]
    fn get_slice(&self, val: &()) -> &[()] {
        if !check() {
            panic!("panic")
        };
        
        let run = || { self.impl_get_slice(val) };
        let ret = run();
        ret
    }
}

fn main() {
    let output;

    let s = S;
    output = s.get_slice(&()).len();

    println!("{}", output);
}
