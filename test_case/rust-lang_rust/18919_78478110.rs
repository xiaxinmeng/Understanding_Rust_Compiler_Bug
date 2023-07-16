 rust
type FuncType<'f> = Fn(&isize) -> isize + 'f;

fn ho_func(f: &mut Option<FuncType>) {
    match f {
        &Some(ref mut f) => {(*f)(&1);},
        &None => {}
    }
}

fn caller() {
    ho_func(&Some(|x| { 1 }));
}

fn main() {}
