rust
mod my_lib {
    pub mod inner_mod {
        pub mod inner_level {
            pub struct A;
            pub struct B;
            pub struct C;
        }
        
        use self::inner_level::B;
        
        pub use self::inner_level::*;
        
        pub fn do_things() {
            let _b = B;
        }
    }
    
    pub use self::inner_mod::*;
}

fn main() {
    let _a = my_lib::A;
    let _c = my_lib::C;
    my_lib::do_things();
}
