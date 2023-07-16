
macro_rules! m {
    (... -> $then:ident!) => {
        $then!(...)
    }
}

macro_rules! then_named! {
    ($($args:tt)*) => {
        named!(f, $($args)*)
    }
}

m!(... -> then_named!);
