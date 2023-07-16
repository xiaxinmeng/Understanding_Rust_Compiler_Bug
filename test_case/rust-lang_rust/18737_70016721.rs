 rust
pub trait Any: 'static {}
pub struct Box2<T: ?Sized>(Box<T>);

impl Box2<Any> {
    pub fn foo(self) {}
}

fn t2(t: Box2<Any + Send>) {
    t.foo(); // type `Box2<Any + Send>` does not implement any method in scope named `foo`
} 
