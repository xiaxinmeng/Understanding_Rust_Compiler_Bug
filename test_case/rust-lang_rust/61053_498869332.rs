
macro_rules! foo {
    ($id:ident) => {
        $id! bar {
            ($id:ident) => {}  // $id is not a binder but an occurrence
        }
    }
}

foo!(macro_rules);

fn main() {
    bar!(macro_rules:ident);  // OK
    bar!(ignore);  // not OK
}
