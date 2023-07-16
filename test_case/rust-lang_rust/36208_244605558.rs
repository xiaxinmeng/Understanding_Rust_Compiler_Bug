
fn f() {}

fn g<'a>() {}

fn main() {
    f::<'static>(); //~ ERROR E0088
    //~^ unexpected lifetime parameter

    g::<'static, 'static>();
}
