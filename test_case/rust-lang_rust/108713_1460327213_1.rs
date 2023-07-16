
($oops:literal, $($args:tt)*) => {
    $crate::panic!("you are missing a writer")
};
