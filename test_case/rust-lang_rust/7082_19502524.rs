
struct Fractal { l: @Fractal, r: @Fractal }

let a = @Fractal { l: _, r: _ }
let b = @Fractal ( l: _, r: _ }
a.l = a;
a.r = b;
b.l = a;
b.r = b;
