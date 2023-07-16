rust
const _DUMMY: () = {
    mod some_mod {
        pub struct Bar;
    }
    use some_mod::Bar;
};
