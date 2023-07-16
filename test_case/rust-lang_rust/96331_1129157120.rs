
fn b<'a, 'b, 'c, 'd>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize, z2: &mut &'d isize) {
    *x = *y;
    *z = *z2;
}
