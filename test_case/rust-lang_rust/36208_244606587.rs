
fn f() {}

fn g<'a>(x: i32) -> &'a str {}

trait Foo<'a> { }

fn main() {
    f::<'static>(); //~ ERROR E0088
    //~^ unexpected lifetime parameter

    g::<'static, 'static>(4);

    let x: Foo<'static, 'static>;
}
