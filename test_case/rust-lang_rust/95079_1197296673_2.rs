
fn main() {
    // Field from upvar
    {
        let mut x = 0;
        || {
            let y = &mut x;
            &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
            *y = 1;
        };
    }
    // Field from upvar nested
    {
        let mut x = 0;
           || {
               || { //~ ERROR captured variable cannot escape `FnMut` closure body
                   let y = &mut x;
                   &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
                   *y = 1;
                   drop(y);
                }
           };
    }
    {
        let f = move || {};
        let _action = move || {
            || f() // The `nested` closure
            //~^ ERROR lifetime may not live long enough
        };
    }
}
