
struct Struct;

impl Struct {
    fn foo(self: &mut Self) {
        (&mut self).bar();
        //~^ ERROR cannot borrow immutable argument `self` as mutable
        //~| HELP to make the argument mutable, use `mut` as shown:
        //~|    fn foo(mut self: &mut Self) {
    }

    fn bar(&mut self) {}
}
