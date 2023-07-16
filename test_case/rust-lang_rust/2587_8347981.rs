


fn foo<T: copy>(+t: T) { ... }

fn bar<T>(+t: T) { ... }


fn main() {
   let v = something_noncopyable();
   foo::<SomethingNoncopyable>(move v); // BAD

   let w = something_noncopyable();
   bar(move w); // OK

   a + b
}
