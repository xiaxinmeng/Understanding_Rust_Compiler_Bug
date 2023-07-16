 rust
#[inline(never)]
fn err_none() -> Result<String, Option<String>> {
    Err(None)
}

#[inline(never)]
fn panic() -> ! {
    panic!()
}

#[inline(always)]
fn ok<T, E>(r: Result<T, E>) -> Option<T> {
    match r {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

fn main() {
    {
        let a = err_none();
        let b = match a {
            Ok(ref x) => Ok(&**x),
            Err(ref e) => Err(e)
        };
        if let Some("a") = ok(b) {
            panic()
        }
    }

    {
        let a = err_none();
        let b = match a {
            Ok(ref x) => Ok(&**x),
            Err(ref e) => Err(e)
        };
        if let Some("a") = ok(b) {
           panic()
        }
    }
}
