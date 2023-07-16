
enum Void { }

fn process(input: *const Void) {
   let _input = unsafe { &*input };
}

fn main() {
    process(std::mem::align_of::<Void>() as *const Void);
}
