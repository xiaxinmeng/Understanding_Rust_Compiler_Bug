rust
macro_rules! foo {
    ($id:ident) => {
        $id! bar {
            ($y:ident) => {}
        }
    }
}

foo!(macro_rules);
