rust
    mod struct_defn {
      pub struct MyTestClass {
          x: i32
      }

      impl MyTestClass {
          pub fn new(x: i32) -> Self {
              Self { x }
          }
      }
    }

    use struct_defn::MyTestClass;

    fn main() {
        let instance = MyTestClass::new(3);
    }
    