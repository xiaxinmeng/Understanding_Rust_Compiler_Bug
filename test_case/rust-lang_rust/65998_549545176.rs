rust
const COUNT: usize = 2;
struct Thing;

const _: () = {
    impl Default for Thing
        where [i32; COUNT]: Default
    {
        fn default() -> Self {
            loop {}
        }
    }
};
