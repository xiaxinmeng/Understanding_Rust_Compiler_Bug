

impl A for i32 {
    type Foo = u32;
}
impl B for u32 {
    const BAR: i32 = 0;
}

trait A {
    type Foo: B;
    
    fn get_val() -> i32 { Self::Foo::get_val() }
}

trait B {
    const BAR: i32;
    
    fn get_val() -> i32 { Self::BAR }
}


fn generic<T: A>() {
    println!("{}", T::get_val());
}

fn main() {
    generic::<i32>();
}
