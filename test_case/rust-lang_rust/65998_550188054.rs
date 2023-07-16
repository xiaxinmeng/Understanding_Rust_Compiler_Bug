rust
trait T { fn t(); }
impl T for [u32; 0] { fn t() {} }

const _: () = {
    impl T for ()
    where
        [u32; 0]: T,
    {
        fn t() {}
    }
};
