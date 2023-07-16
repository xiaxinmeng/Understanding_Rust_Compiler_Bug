
fn foo<T>(x: *const T) -> Box<T>{
type ConcreteRaw = *const T;
type Concrete = Box<T>;
transmute::<ConcreteRaw, Concrete>(x);
}
