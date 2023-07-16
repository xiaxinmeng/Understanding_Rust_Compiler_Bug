
struct Ref<'a> { x: &'a u32 }

fn foo(mut x: Vec<Ref>, y: Ref) {
                  ---      --- the reference and struct are not declared with the same lifetime ...
  x.push(y);
|            ^ data from `y` flows into `x` here

}
