
#[macro_export]
macro_rules! problem {
    () => {
        #[derive(Print)]
        pub struct S(<MyDispatch as $crate::Dispatch::Second<Test>>::Call);
    }
}
