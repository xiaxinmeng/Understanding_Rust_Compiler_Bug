rust
fn generic_over_return_type<T: SomeTrait>(foo: &Foo, bar: &Bar) -> T {
    /* create a T */
}

let var: ConcreteType<'_, '_> = generic_over_return_type(&foo, &bar);
