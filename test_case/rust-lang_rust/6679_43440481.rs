 rust
let value = cascade!(Foo::new()
                     .. add_to_y(2.0)
                     .. x = 2
                     .. double_x()
                     .. z = "bar".to_strbuf());
