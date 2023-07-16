rust
enum Void { }

fn process(input: *const Void) {
   let _input = unsafe { &*input };
}

fn main() {}
