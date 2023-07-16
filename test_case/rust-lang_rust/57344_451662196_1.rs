rust
extern crate aux;

fn get_default<T: Default>(_t: &aux::Pub<T>) -> T { T::default() }

fn main() {
    let pub_ = aux::Pub::Nothing;
    aux::assert_aux(&pub_);
    let def = get_default(&pub_);
    def.pub_method();
}
