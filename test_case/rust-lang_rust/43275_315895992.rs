
struct Ref<'a, 'b> { a: &'a u32, b: &'b u32 }

fn foo(mut x: Ref) {
              ---
              1st lifetime parameter on `Ref` must match
              2nd lifetime parameter on `Ref` must match
  x.a = x.b;
}

