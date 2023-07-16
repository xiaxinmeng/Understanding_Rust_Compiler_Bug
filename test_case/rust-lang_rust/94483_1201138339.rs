rust
fn main() {
    |var: _| { // `_` placeholder is necessary.
    //^ Past : consider giving this closure parameter a type
        var.abc()
        //^ Present : cannot infer type
    };
}
