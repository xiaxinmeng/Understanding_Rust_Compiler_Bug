
pub enum Foo<T> {
    A(),
    B(T),
}

pub struct Bar {}

pub struct User {
    pub data: RefCell<Foo<Bar>>,
}

impl User {
    pub fn get_data(&self) -> std::result::Result<Ref<'_, Bar>, String> {
        let v = self.data.borrow();
        if let Foo::B(_) = *v {
            Ok(Ref::map(v, |x| match x {
                Foo::B(b) => b,
                _ => unreachable!(),
            }))
        } else {
            Err(String::from("error"))
        }
    }
}
