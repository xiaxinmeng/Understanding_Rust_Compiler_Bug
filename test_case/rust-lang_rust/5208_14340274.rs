
let (x,y) = (@mut 5, @mut 7);
let (xp,yp) = (&mut *x, &mut *y);
assert xp.lt(yp);
make_bigger(x,y);
assert yp.lt(xp);

fn make_bigger(x: @mut int, yp: @mut int) {
    if (x.lt(y)) { *x = *y + 1 }
}
