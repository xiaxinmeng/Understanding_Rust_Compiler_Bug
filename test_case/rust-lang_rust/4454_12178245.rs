
struct Foo { f: int };
let x = @mut @mut Foo { f: 2 };
let y = &mut x.f;
x.f += 2; // (1)
