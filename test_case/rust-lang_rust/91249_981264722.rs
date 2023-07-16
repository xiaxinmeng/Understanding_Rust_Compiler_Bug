rust
macro define_struct($name: ident, $field_name: ident, $make_name: ident, { $($defn: item)* }) {
    struct $name { $field_name: i32 }

    impl $name {
        pub fn secret() -> Self {
            $name { $field_name: 42 }
        }

        $($defn)*
    }

    fn $make_name() -> $name {
        $name::secret()
    }
}

define_struct!(S, x, make_s, {
    pub fn get_x(self) -> i32 {
        self.x
    }
});

fn main() {
    println!("{}", make_s().get_x());
}
