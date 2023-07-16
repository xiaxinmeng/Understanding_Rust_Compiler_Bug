
#![feature(unboxed_closures, unboxed_closure_sugar)]

fn bug() {
  let c = box |&:| {};
}
